//! Node info middleware.

use iron::prelude::*;
use iron::{typemap, BeforeMiddleware};

pub struct NodeMiddleware {
    address: String
}

impl typemap::Key for NodeMiddleware {
    type Value = String;
}

impl NodeMiddleware {
    pub fn new(address: String) -> NodeMiddleware {
        NodeMiddleware {
            address: address
        }
    }
}

impl BeforeMiddleware for NodeMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<NodeMiddleware>(self.address.clone());
        Ok(())
    }
}

pub trait NodeExtractor {
    fn get_node_address(&self) -> IronResult<String>;
}

impl<'a, 'b> NodeExtractor for Request<'a, 'b> {
    fn get_node_address(&self) -> IronResult<String> {
        match self.extensions.get::<NodeMiddleware>() {
            Some(address) => Ok(address.clone()),
            None => error!(InternalServerError, "No node address is embedded in this request")
        }
    }
}