//! API Command Line Interface.

use error::*;
use api::{Server, router};

/// Start the API daemon.
pub fn start_daemon(listen_addr: String, protected_mode_active: bool) -> LocksidianResult<String> {
	let server = Server::new(listen_addr, protected_mode_active);
	
	server.start(router())
}