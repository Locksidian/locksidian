//! HTTP REST API Server
//!
//! Launch the server daemon using either the `--daemon={listen_addr}` command line argument or the
//! `LS_DAEMON={listen_addr}` environment variable.

use iron::prelude::*;
use iron::Handler;

use super::middleware;
use persistence::database_path;

/// HTTP server exposing the `Locksidian` REST API.
pub struct Server {

    /// Address on which the HTTP server will be listening.
    /// Use `0.0.0.0` in order to listen on any IP addresses/DNS that reaches your node.
    listen_addr: String
}

impl Server {

    /// Create a new `Server` instance.
    pub fn new(listen_addr: String) -> Server {
        Server {
            listen_addr: listen_addr
        }
    }

    /// Configure the middlewares wrapping every routes.
    /// Used to add new behavior before, around and after each requests/responses.
    fn chain<H: Handler>(&self, handler: H) -> Chain {
        let mut chain = Chain::new(handler);

        chain.link_before(
            middleware::PoolMiddleware::new(database_path())
                .expect("Unable to create a connection pool")
        );
        chain.link_after(middleware::HeadersMiddleware);

        chain
    }

    /// Starts the API server by binding the request chain to the provided `handler` and listening
    /// on the configured address.
    pub fn start<H: Handler>(&self, handler: H) {
        let chain = self.chain(handler);
        let server = Iron::new(chain).http(self.listen_addr.as_str());

        match server {
            Ok(_) => println!("Locksidian daemon listening on: {}", self.listen_addr),
            Err(err) => panic!(err.to_string())
        }
    }
}