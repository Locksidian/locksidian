//! Peers API endpoints.

use iron::prelude::*;
use persistence::prelude::*;

use api::middleware::node::NodeExtractor;

use blockchain::peer::*;

pub fn get_all(req: &mut Request) -> IronResult<Response> {
	let connection = req.get_connection()?;
	let repository = PeerRepository::new(&*connection);
	
	match repository.get_all() {
		Some(entities) => {
			let peers: Vec<PeerDto> = entities.iter()
				.map(|entity| Peer::from_entity(entity))
				.filter(|peer| peer.is_ok())
				.map(|peer| peer.unwrap())
				.map(|peer| PeerDto::new(&peer))
				.filter(|dto| dto.is_ok())
				.map(|dto| dto.unwrap())
				.collect();
			
			response!(Ok, peers)
		},
		None => response!(NoContent, {})
	}
}

pub fn register(req: &mut Request) -> IronResult<Response> {
    let mut peer = body_to_peer(req)?;
    let connection = req.get_connection()?;
    let repository = PeerRepository::new(&*connection);
    let address = req.get_node_address()?;

    info!("Attempting to register {} peer at {}", peer.identity(), peer.address());

    match peer_cli::register(&mut peer, &repository, address.as_ref()) {
        Ok(_) => match peer_cli::current_identity_as_peer(&*connection, address) {
            Ok(peer) => match PeerDto::new(&peer) {
                Ok(dto) => {
                    info!("Successfully registered peer {} at {}", peer.identity(), peer.address());
                    response!(Ok, dto)
                },
                Err(err) => {
                    warn!("Could not create peer {} at {}", peer.identity(), peer.address());
                    response!(InternalServerError, {"error": err.description()})
                }
            },
            Err(err) => {
                warn!("Could not convert current identity as peer using address {}", req.get_node_address()?);
                response!(InternalServerError, {"error": err.description()})
            }
        },
        Err(err) => {
            warn!("Could not register peer {} at {}", peer.identity(), peer.address());
            response!(InternalServerError, {"error": err.description()})
        }
    }
}

fn body_to_peer(req: &mut Request) -> IronResult<Peer> {
    let dto = body_to_dto(req)?;
    
    match dto.to_peer() {
        Ok(peer) => Ok(peer),
        Err(err) => http_error!(BadRequest, {"error": err.description()})
    }
}

fn body_to_dto(req: &mut Request) -> IronResult<PeerDto> {
    match body!(req, PeerDto) {
        Ok(Some(dto)) => Ok(dto),
        Ok(None) => http_error!(BadRequest, {"error": "No content"}),
        Err(err) => http_error!(BadRequest, {"error": err.description()})
    }
}