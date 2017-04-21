//! HTTP REST API Server
//!
//! Launch the server daemon using either the `--daemon={listen_addr}` command line argument or the
//! `LS_DAEMON={listen_addr}` environment variable.

use iron::prelude::*;
use iron::Handler;
use iron::Listening;

use persistence::prelude::*;
use api::middleware::*;

use blockchain::identity::*;

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
				
                match self.on_start() {
					Ok(_) => Ok(String::from("Daemon initialization successful!")),
					Err(msg) => {
						self.stop(&mut listener)?;
						Err(msg)
					}
				}
            },
            Err(err) => Err(err.to_string())
        }
    }

    /// Callback method called when the `Locksidian` server starts.
    fn on_start(&self) -> Result<(), String> {
		let connection = self.setup_database()?;
		
		let identity = cli::get_active_identity(&connection)?;
		println!("Startup identity is: {}", identity.hash());
		
		Ok(())
    }
    
    fn setup_database(&self) -> Result<SqliteConnection, String> {
        let connection = get_connection(database_path())?;
        setup_database(&connection)?;
		
		Ok(connection)
    }

    /// Gracefully stops the running `Listening` instance.
    fn stop(&self, listener: &mut Listening) -> Result<String, String> {
        match listener.close() {
            Ok(_) => Ok(String::from("Locksidian daemon stopped gracefully")),
            Err(err) => Err(err.to_string())
        }
    }
}