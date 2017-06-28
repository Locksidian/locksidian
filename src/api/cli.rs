//! API Command Line Interface.

use error::*;
use api::{Server, ServerConfig, router};

use blockchain::network::to_ipv4_socket;

/// Start the API daemon.
pub fn start_daemon(listen_addr: String, config: ServerConfig) -> LocksidianResult<String> {
	let socket = to_ipv4_socket(listen_addr)?;
	let server = Server::new(socket, config);
	
	server.start(router())
}