//! Peers API endpoints.

use iron::prelude::*;
use persistence::prelude::*;

use api::middleware::node::NodeExtractor;

use blockchain::peer::*;

pub fn register(req: &mut Request) -> IronResult<Response> {
    let mut peer = body_to_peer(req)?;
    let connection = req.get_connection()?;
    let repository = PeerRepository::new(&*connection);

    match peer_cli::register(&mut peer, &repository) {
        Ok(_) => {
            let address = req.get_node_address()?;
            
            match peer_cli::current_identity_as_peer(&*connection, address) {
                Ok(peer) => match PeerDto::new(&peer) {
                    Ok(dto) => response!(Ok, {"peer": dto}),
                    Err(err) => response!(InternalServerError, {"error": err.description()})
                },
                Err(err) => response!(InternalServerError, {"error": err.description()})
            }
        },
        Err(err) => response!(InternalServerError, {"error": err.description()})
    }
}

fn body_to_peer(req: &mut Request) -> IronResult<Peer> {
    let dto = body_to_dto(req)?;
    
    match dto.to_peer() {
        Ok(peer) => Ok(peer),
        Err(err) => error!(BadRequest, {"error": err.description()})
    }
}

fn body_to_dto(req: &mut Request) -> IronResult<PeerDto> {
    match body!(req, PeerDto) {
        Ok(Some(dto)) => Ok(dto),
        Ok(None) => error!(BadRequest, {"error": "No content"}),
        Err(err) => error!(BadRequest, {"error": err.description()})
    }
}