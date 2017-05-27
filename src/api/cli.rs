//! API Command Line Interface.

use error::*;
use api::{Server, router};

/// Start the API daemon.
pub fn start_daemon(listen_addr: String, protected: bool, entrypoint: Option<String>) -> LocksidianResult<String> {
	let server = Server::new(listen_addr, protected, entrypoint);
	
	server.start(router())
}