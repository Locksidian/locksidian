//! Blocks management endpoint.

use iron::prelude::*;
use persistence::prelude::*;

use blockchain::peer::*;
use blockchain::network::*;

use blockchain::identity::*;
use blockchain::block::*;

pub fn preflight(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::with((::iron::status::Ok, ""));
    
    res.headers.set_raw("Access-Control-Allow-Methods", vec![Vec::from(
        "GET, POST".as_bytes()
    )]);
    res.headers.set_raw("Access-Control-Allow-Headers", vec![Vec::from(
        "Content-Type".as_bytes()
    )]);
    
    Ok(res)
}

/// Store the provided `Request` body in a new `Block` inside the Locksidian blockchain.
///
/// The generated block hash is then returned to the client:
///
/// ```json
/// {
///     "block": "{hash}"
/// }
/// ```
pub fn store_document(req: &mut Request) -> IronResult<Response> {
    match body_raw!(req) {
        Ok(Some(body)) => {
            let connection = req.get_connection()?;
            let identity = get_active_identity(&*connection)?;
            let repository = BlockRepository::new(&*connection);
    
            match Block::new(body, &identity, &repository) {
                Ok(block) => {
                    let entity = BlockEntity::new(&block);
            
                    match repository.save_head(&entity) {
                        Ok(1) => {
	                        let peer_repository = PeerRepository::new(&*connection);
                            propagate_block(&block, &peer_repository, &*connection)?;
	                        
                            http_response!(Ok, {"block": block.hash()})
                        },
                        Ok(_) => http_response!(InternalServerError, {
                                    "warning": "An unexpected number of rows were inserted in the registry"
                                }),
                        Err(err) => http_response!(InternalServerError, {"error": err.description()})
                    }
                },
                Err(err) => http_response!(Conflict, {"error": err.description()})
            }
        },
        Ok(None) => http_response!(BadRequest, {"error": "Request body cannot be null"}),
        Err(err) => http_response!(InternalServerError, {"error": err.to_string()})
    }
}

/// Return the hash of the current blockchain `HEAD` block:
///
/// ```json
/// {
///     "head": "{hash}"
/// }
/// ```
pub fn show_head(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_connection()?;
    let repository = BlockRepository::new(&*connection);

    match repository.get_head() {
        Some(head) => http_response!(Ok, {"head": head.hash}),
        None => http_response!(NoContent, {})
    }
}

/// Get all the `Block` data of the block identitfied by the provided `hash`.
pub fn get_block(req: &mut Request) -> IronResult<Response> {
    match route_param!(req, "hash") {
        Some(hash) => {
            let connection = req.get_connection()?;
            let repository = BlockRepository::new(&*connection);

            match repository.get(&String::from(hash)) {
                Some(entity) => match Block::from_entity(entity) {
                    Ok(block) => {
                        let dto = BlockDto::new(&block);
                        http_response!(Ok, dto)
                    },
                    Err(err) => http_response!(InternalServerError, {"error": err.description()})
                },
                None => http_response!(NoContent, {})
            }
        },
        None => http_response!(BadRequest, {"error": "Hash parameter cannot be empty"})
    }
}

/// Create a local copy of the `Block` if its structure is valid.
pub fn replicate_block(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_connection()?;
    let block_repository = BlockRepository::new(&*connection);
	let peer_repository = PeerRepository::new(&*connection);
    
    let mut block = body_to_block(req, &block_repository)?;
    let should_sync = save_replicated_block(&mut block, &block_repository)?;
    propagate_block(&block, &peer_repository, &*connection)?;
	
	if should_sync {
		match peer_repository.get(&block.received_from()) {
			Some(entity) => match Peer::from_entity(&entity) {
				Ok(peer) => HttpClient::from_peer(&peer).sync(Some(block.previous()), &block_repository).unwrap_or(()),
				Err(_) => ()
			},
			None => ()
		};
	}
	
    http_response!(Ok, {})
}

/// Propagate a `Block` to all of our `Peer`s.
fn propagate_block(block: &Block, repository: &PeerRepository, connection: &SqliteConnection) -> IronResult<()> {
    let identity = get_active_identity(&*connection)?;
    
    match repository.get_all() {
        Some(entities) => {
            let peers: Vec<Peer> = entities.iter()
                .map(|entity| Peer::from_entity(entity))
                .filter(|peer| peer.is_ok())
                .map(|peer| peer.unwrap())
                .collect();
            
            match HttpClient::propagate(&block, &identity, peers) {
                Ok(_) => Ok(()),
                Err(_) => Ok(())
            }
        },
        None => http_error!(InternalServerError, {"error": "No peer could be found to propagate this block"})
    }
}

fn save_replicated_block(block: &mut Block, repository: &BlockRepository) -> IronResult<bool> {
    let mut entity = BlockEntity::new(&block);
	let mut should_sync = false;
    
    let save = match repository.get(&block.previous()) {
        Some(mut previous) => repository.save_next(&mut entity, &mut previous),
        None => {
	        should_sync = true;
	        repository.save(&entity)
        }
    };
    
    match save {
        Ok(1) => Ok(should_sync),
        Ok(_) => http_error!(InternalServerError, {
            "warning": "An unexpected number of rows were inserted in the registry"
        }),
        Err(err) => http_error!(InternalServerError, {"error": err.description()})
    }
}

fn get_active_identity(connection: &SqliteConnection) -> IronResult<Identity> {
    match identity_cli::get_active_identity(&connection) {
        Ok(identity) => Ok(identity),
        Err(err) => http_error!(InternalServerError, {"error": err.description()})
    }
}

fn body_to_block(req: &mut Request, repository: &BlockRepository) -> IronResult<Block> {
    let dto = body_to_dto(req)?;
    
    match Block::replicate_from(dto, &repository) {
        Ok(block) => Ok(block),
        Err(err) => http_error!(BadRequest, {"error": err.description()})
    }
}

fn body_to_dto(req: &mut Request) -> IronResult<BlockReplicationDto> {
    match body!(req, BlockReplicationDto) {
        Ok(Some(dto)) => Ok(dto),
        Ok(None) => http_error!(BadRequest, {"error": "No content"}),
        Err(err) => http_error!(BadRequest, {"error": err.description()})
    }
}