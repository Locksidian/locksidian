//! API Command Line Interface.

use api::*;
use persistence::*;

/// Start the API daemon.
pub fn start_daemon(listen_addr: String) -> Result<String, String> {
	let connection = get_connection(database_path())?;
	setup_database(&connection)?;
	
	let server = Server::new(listen_addr);
	server.start(router());

	Ok(String::from("API server stopped gracefully"))
}