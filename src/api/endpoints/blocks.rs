//! Blocks management endpoint.

use iron::prelude::*;
use persistence::prelude::*;

use blockchain::peer::*;
use blockchain::network::*;

use blockchain::identity::*;
use blockchain::block::*;

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
                            propagate_block(&block, &*connection)?;
                            response!(Ok, {"block": block.hash()})
                        },
                        Ok(_) => response!(InternalServerError, {
                                    "warning": "An unexpected number of rows were inserted in the registry"
                                }),
                        Err(err) => response!(InternalServerError, {"error": err.description()})
                    }
                },
                Err(err) => response!(Conflict, {"error": err.description()})
            }
        },
        Ok(None) => response!(BadRequest, {"error": "Request body cannot be null"}),
        Err(err) => response!(InternalServerError, {"error": err.to_string()})
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
        Some(head) => response!(Ok, {"head": head.hash}),
        None => response!(NoContent, {})
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
                        response!(Ok, {"block": dto})
                    },
                    Err(err) => response!(InternalServerError, {"error": err.description()})
                },
                None => response!(NoContent, {})
            }
        },
        None => response!(BadRequest, {"error": "Hash parameter cannot be empty"})
    }
}

/// Create a local copy of the `Block` if its structure is valid.
pub fn replicate_block(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_connection()?;
    let repository = BlockRepository::new(&*connection);
    
    let mut block = body_to_block(req, &repository)?;
    save_replicated_block(&mut block, &repository)?;
    
    propagate_block(&block, &*connection)?;
    response!(Ok, {})
}

/// Propagate a `Block` to all of our `Peer`s.
fn propagate_block(block: &Block, connection: &SqliteConnection) -> IronResult<()> {
    let identity = get_active_identity(&*connection)?;
    
    let repository = PeerRepository::new(&connection);
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
        None => error!(InternalServerError, {"error": "No peer could be found to propagate this block"})
    }
}

fn save_replicated_block(block: &mut Block, repository: &BlockRepository) -> IronResult<()> {
    let mut entity = BlockEntity::new(&block);
    
    let save = match repository.get(&block.previous()) {
        Some(mut previous) => repository.save_next(&mut entity, &mut previous),
        None => repository.save(&entity)
    };
    
    match save {
        Ok(1) => Ok(()),
        Ok(_) => error!(InternalServerError, {
            "warning": "An unexpected number of rows were inserted in the registry"
        }),
        Err(err) => error!(InternalServerError, {"error": err.description()})
    }
}

fn get_active_identity(connection: &SqliteConnection) -> IronResult<Identity> {
    match identity_cli::get_active_identity(&connection) {
        Ok(identity) => Ok(identity),
        Err(err) => error!(InternalServerError, {"error": err.description()})
    }
}

fn body_to_block(req: &mut Request, repository: &BlockRepository) -> IronResult<Block> {
    let dto = body_to_dto(req)?;
    
    match Block::replicate_from(dto, &repository) {
        Ok(block) => Ok(block),
        Err(err) => error!(BadRequest, {"error": err.description()})
    }
}

fn body_to_dto(req: &mut Request) -> IronResult<BlockReplicationDto> {
    match body!(req, BlockReplicationDto) {
        Ok(Some(dto)) => Ok(dto),
        Ok(None) => error!(BadRequest, {"error": "No content"}),
        Err(err) => error!(BadRequest, {"error": err.description()})
    }
}