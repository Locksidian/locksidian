//! API Command Line Interface.

use api::{Server, router};

/// Start the API daemon.
pub fn start_daemon(listen_addr: String) -> Result<String, String> {
	let server = Server::new(listen_addr);
	
	server.start(router())
}