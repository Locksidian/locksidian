//! API Command Line Interface.

use opts;
use api::*;
use persistence::*;

/// Start the API daemon.
pub fn start_daemon(opt_addr: Option<String>) {
	match opt_addr {
		Some(listen_addr) => {
			match get_connection(database_path()) {
				Ok(connection) => setup_database(&connection).expect("Unable to initialize the database schemas"),
				Err(msg) => panic!(msg)
			}
			
			let server = Server::new(listen_addr);
			server.start(router());
		},
		None => println!("{}", opts::usage())
	}
}