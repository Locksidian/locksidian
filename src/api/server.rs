//! HTTP REST API Server
//!
//! Launch the server daemon using either the `--daemon={listen_addr}` command line argument or the
//! `LS_DAEMON={listen_addr}` environment variable.

use iron::prelude::*;
use iron::Handler;
use iron::Listening;

use persistence::*;
use api::middleware::*;

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
    fn configure_middlewares<H: Handler>(&self, handler: H) -> Result<Chain, String> {
        let mut chain = Chain::new(handler);

        chain.link_before(PoolMiddleware::new(database_path())?);
        chain.link_before(ClientMiddleware::new());
        chain.link_after(HeadersMiddleware);

        Ok(chain)
    }

    /// Starts the API server by binding the request chain to the provided `handler` and listening
    /// on the configured address.
    pub fn start<H: Handler>(&self, handler: H) -> Result<String, String> {
        let chain = self.configure_middlewares(handler)?;
        let status = Iron::new(chain).http(self.listen_addr.as_str());

        match status {
            Ok(mut listener) => {
                println!("Locksidian daemon listening on: {}", self.listen_addr);
                self.on_start(&mut listener)
            },
            Err(err) => Err(err.to_string())
        }
    }

    /// Callback method called when the `Locksidian` server starts.
    fn on_start(&self, listener: &mut Listening) -> Result<String, String> {
        // TODO: check if there is an active Identity; stop the server if this is not the case.
        self.stop(listener)
    }

    /// Gracefully stops the running `Listening` instance.
    fn stop(&self, listener: &mut Listening) -> Result<String, String> {
        match listener.close() {
            Ok(_) => Ok(String::from("Locksidian daemon stopped gracefully")),
            Err(err) => Err(err.to_string())
        }
    }
}