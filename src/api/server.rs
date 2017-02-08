//! API Server

use iron::{Iron, Chain};
use router::Router;

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

    fn chain(&self, router: Router) -> Chain {
        let mut chain = Chain::new(router);

        chain.link_after(middleware::HeadersMiddleware);

        chain
    }

    pub fn start(&self, router: Router) {
        let chain = self.chain(router);
        let server = Iron::new(chain).http(self.listen_addr.as_str());

        match server {
            Ok(_) => println!("Listening on {}!", self.listen_addr),
            Err(e) => panic!(e)
        }
    }
}