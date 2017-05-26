//! HTTP REST API Server
//!
//! Launch the server daemon using either the `--daemon={listen_addr}` command line argument or the
//! `LS_DAEMON={listen_addr}` environment variable.

use error::*;

use iron::prelude::*;
use iron::Handler;
use iron::Listening;

use persistence::prelude::*;
use api::middleware::*;

use blockchain::identity::identity_cli::get_active_identity;

/// HTTP server exposing the `Locksidian` REST API.
pub struct Server {

    /// Address on which the HTTP server will be listening.
    /// Use `0.0.0.0` in order to listen on any IP addresses/DNS that reaches your node.
    listen_addr: String,

    /// Is the protected mode activated for this `Server` instance?
    protected_mode_active: bool
}

impl Server {

    /// Create a new `Server` instance.
    pub fn new(listen_addr: String, protected_mode_active: bool) -> Server {
        Server {
            listen_addr: listen_addr,
            protected_mode_active: protected_mode_active
        }
    }

    /// Configure the middlewares wrapping every routes.
    /// Used to add new behavior before, around and after each requests/responses.
    fn configure_middlewares<H: Handler>(&self, handler: H) -> LocksidianResult<Chain> {
        let mut chain = Chain::new(handler);

        chain.link_before(NodeMiddleware::new(self.listen_addr()));
        chain.link_before(PoolMiddleware::new(database_path())?);

        if self.protected_mode_active {
            chain.link_before(ProtectedMiddleware::new());
        }

        chain.link_before(ClientMiddleware::new());
        chain.link_after(HeadersMiddleware);

        Ok(chain)
    }

    /// Starts the API server by binding the request chain to the provided `handler` and listening
    /// on the configured address.
    pub fn start<H: Handler>(&self, handler: H) -> LocksidianResult<String> {
        let chain = self.configure_middlewares(handler)?;
        let status = Iron::new(chain).http(self.listen_addr.as_str());

        match status {
            Ok(mut listener) => {
                println!("Locksidian daemon listening on: {}", self.listen_addr);
				
                match self.on_start() {
					Ok(_) => Ok(String::from("Daemon initialization successful!")),
					Err(err) => {
						self.stop(&mut listener)?;
						Err(err)
					}
				}
            },
            Err(err) => Err(LocksidianError::from_err(err))
        }
    }

    /// Callback method called when the `Locksidian` server starts.
    fn on_start(&self) -> LocksidianResult<()> {
		let connection = self.setup_database()?;
		
		let identity = get_active_identity(&connection)?;
		println!("Startup identity is: {}", identity.hash());
		
		Ok(())
    }
    
    fn setup_database(&self) -> LocksidianResult<SqliteConnection> {
        let connection = get_connection(database_path())?;
        setup_database(&connection)?;
		
		Ok(connection)
    }

    /// Gracefully stops the running `Listening` instance.
    fn stop(&self, listener: &mut Listening) -> LocksidianResult<String> {
        match listener.close() {
            Ok(_) => Ok(String::from("Locksidian daemon stopped gracefully")),
            Err(err) => Err(LocksidianError::from_err(err))
        }
    }

    pub fn listen_addr(&self) -> String {
        self.listen_addr.clone()
    }
}