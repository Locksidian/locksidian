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

use blockchain::identity::Identity;
use blockchain::identity::identity_cli::get_active_identity;

use blockchain::network::*;
use blockchain::peer::*;

/// HTTP server exposing the `Locksidian` REST API.
pub struct Server {

    /// Address on which the HTTP server will be listening.
    /// Use `0.0.0.0` in order to listen on any IP addresses/DNS that reaches your node.
    listen_addr: String,

    /// Is the protected mode activated for this `Server` instance?
    protected: bool,
    
    /// Optional network entrypoint IP address or hostname
    entrypoint: Option<String>
}

impl Server {

    /// Create a new `Server` instance.
    pub fn new(listen_addr: String, protected: bool, entrypoint: Option<String>) -> Server {
        Server {
            listen_addr: listen_addr,
            protected: protected,
			entrypoint: entrypoint
        }
    }

    /// Configure the middlewares wrapping every routes.
    /// Used to add new behavior before, around and after each requests/responses.
    fn configure_middlewares<H: Handler>(&self, handler: H) -> LocksidianResult<Chain> {
        let mut chain = Chain::new(handler);

        chain.link_before(NodeMiddleware::new(self.listen_addr()));
        chain.link_before(PoolMiddleware::new(database_path())?);

        if self.protected {
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
	
	/// Gracefully stops the running `Listening` instance.
	fn stop(&self, listener: &mut Listening) -> LocksidianResult<String> {
		match listener.close() {
			Ok(_) => Ok(String::from("Locksidian daemon stopped gracefully")),
			Err(err) => Err(LocksidianError::from_err(err))
		}
	}

    /// Callback method called when the `Locksidian` server starts.
    fn on_start(&self) -> LocksidianResult<()> {
		let connection = self.setup_database()?;
		let identity = self.setup_identity(&connection)?;
		
		self.setup_network(&connection, &identity)?;
		
		Ok(())
    }
    
	/// Establish a connection to the registry and setup the database schemas.
    fn setup_database(&self) -> LocksidianResult<SqliteConnection> {
        let connection = get_connection(database_path())?;
        setup_database(&connection)?;
		
		Ok(connection)
    }
	
	/// Gather and return the currently configured `Identity`.
	fn setup_identity(&self, connection: &SqliteConnection) -> LocksidianResult<Identity> {
		let identity = get_active_identity(&connection)?;
		println!("Startup identity is: {}", identity.hash());
		
		Ok(identity)
	}
	
	/// Setup the Locksidian network by establishing a connection to the server's `entrypoint` or by
	/// starting a new network on its own.
	fn setup_network(&self, connection: &SqliteConnection, identity: &Identity) -> LocksidianResult<()> {
		match self.entrypoint {
			Some(ref entrypoint) => {
				let client = HttpClient::from_address(entrypoint.clone());
				let repository = PeerRepository::new(&connection);
				
				self.network_registration(&client, &identity, &repository)?;
				self.register_network_peers(&client, &repository)?;
				
				println!("Successfully registered onto the network.");
			},
			None => println!("Standalone network mode. Entrypoint is: {}", self.listen_addr)
		}
		
		Ok(())
	}
	
	/// Try to establish a connection and register our instance with the network entrypoint.
	fn network_registration<T: Client>(&self, client: &T, identity: &Identity, repository: &PeerRepository) -> LocksidianResult<()> {
		let key = identity.public_key_to_hex()?;
		let peer = Peer::new(key, self.listen_addr())?;
		
		match client.register(&peer) {
			Ok(mut peer) => peer_cli::register(&mut peer, &repository),
			Err(err) => Err(LocksidianError::from_err(err))
		}
	}
	
	/// If the registration process is successfull, we gather the `Peer`s list to update our registry.
	fn register_network_peers<T: Client>(&self, client: &T, repository: &PeerRepository) -> LocksidianResult<()> {
		let mut peers = client.get_peers()?;
		peer_cli::register_batch(&mut peers, &repository)
	}

	/// `listen_addr` getter.
    pub fn listen_addr(&self) -> String {
        self.listen_addr.clone()
    }
}