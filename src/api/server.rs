//! HTTP REST API Server
//!
//! Launch the server daemon using either the `--daemon={listen_addr}` command line argument or the
//! `LS_DAEMON={listen_addr}` environment variable.

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
            Ok(_) => println!("Locksidian daemon listening on: {}", self.listen_addr),
            Err(err) => panic!(err.to_string())
        }
    }
}