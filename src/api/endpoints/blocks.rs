//! Blocks management endpoint.

use iron::prelude::*;
use persistence::prelude::*;

use blockchain::identity::*;
use blockchain::block::*;

pub fn store_document(req: &mut Request) -> IronResult<Response> {
    match body_raw!(req) {
        Ok(Some(body)) => match req.get_connection() {
            Ok(connection) => match cli::get_active_identity(&*connection) {
                Ok(identity) => {
                    let repository = BlockRepository::new(&*connection);
                    
                    match Block::new(body, &identity, &repository) {
                        Ok(block) => {
                            let entity = BlockEntity::new(&block);

                            match repository.save_head(&entity) {
                                Ok(1) => response!(Ok, {"block": block.hash()}),
                                Ok(_) => response!(InternalServerError, {
                                    "warning": "An unexpected number of rows were inserted in the registry"
                                }),
                                Err(msg) => response!(InternalServerError, {"error": msg})
                            }
                        },
                        Err(msg) => response!(Conflict, {"error": msg})
                    }
                },
                Err(msg) => response!(InternalServerError, {"error": msg})
            },
            Err(msg) => response!(InternalServerError, {"error": msg})
        },
        Ok(None) => response!(BadRequest, {"error": "Request body cannot be null"}),
        Err(err) => response!(InternalServerError, {"error": err.to_string()})
    }
}