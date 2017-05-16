//! Blocks management endpoint.

use iron::prelude::*;
use persistence::prelude::*;

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
                                Err(err) => response!(InternalServerError, {"error": err.description()})
                            }
                        },
                        Err(err) => response!(Conflict, {"error": err.description()})
                    }
                },
                Err(err) => response!(InternalServerError, {"error": err.description()})
            },
            Err(err) => response!(InternalServerError, {"error": err.description()})
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
    match req.get_connection() {
        Ok(connection) => {
            let repository = BlockRepository::new(&*connection);

            match repository.get_head() {
                Some(head) => response!(Ok, {"head": head.hash}),
                None => response!(NoContent, {})
            }
        },
        Err(err) => response!(InternalServerError, {"error": err.description()})
    }
}

/// Get all the `Block` data of the block identitfied by the provided `hash`.
pub fn get_block(req: &mut Request) -> IronResult<Response> {
    match route_param!(req, "hash") {
        Some(hash) => match req.get_connection() {
           Ok(connection) => {
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
            Err(err) => response!(InternalServerError, {"error": err.description()})
        },
        None => response!(BadRequest, {"error": "Hash parameter cannot be emtpy"})
    }
}