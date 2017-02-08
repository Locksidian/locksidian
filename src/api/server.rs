//! API Server

use iron::prelude::*;
use iron::Handler;

use super::middleware;

pub struct Server {
    listen_addr: String
}

impl Server {
    pub fn new(listen_addr: String) -> Server {
        Server {
            listen_addr: listen_addr
        }
    }

    fn chain<H: Handler>(&self, handler: H) -> Chain {
        let mut chain = Chain::new(handler);

        chain.link_after(middleware::HeadersMiddleware);

        chain
    }

    pub fn start<H: Handler>(&self, handler: H) {
        let chain = self.chain(handler);
        let server = Iron::new(chain).http(self.listen_addr.as_str());

        match server {
            Ok(_) => println!("Listening on {}!", self.listen_addr),
            Err(e) => panic!(e)
        }
    }
}