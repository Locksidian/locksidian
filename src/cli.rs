//! Command Line Interface.
//!
//! Handle the command line arguments provided by the `opts` module and the environment variables
//! defined in the system/Docker container.

use error::*;

use getopts::Matches;
use opts;

use api;
use blockchain::identity::identity_cli;

pub fn handle(matches: Matches) -> LocksidianResult<String> {
	// Generic options
    if matches.opt_present("help") {
        Ok(opts::usage())
    }
    else if matches.opt_present("version") {
        Ok(opts::version())
    }
	// API
    else if matches.opt_present("daemon") {
        match matches.opt_str("daemon") {
            Some(addr) => api::cli::start_daemon(addr, matches.opt_present("protected")),
            None => Err(LocksidianError::new(opts::usage()))
        }
    }
    else if opts::env("LS_DAEMON").is_some() {
        match opts::env("LS_DAEMON") {
            Some(addr) => api::cli::start_daemon(addr, matches.opt_present("protected")),
            None => Err(LocksidianError::new(opts::usage()))
        }
    }
	// Identity
    else if matches.opt_present("identity") {
        match matches.opt_str("identity") {
            Some(hash) => identity_cli::set_active_identity(hash),
            None => Err(LocksidianError::new(opts::usage()))
        }
    }
	else if matches.opt_present("identity-new") {
		match matches.opt_str("identity-new") {
			Some(bit_size) => identity_cli::generate_new_identity(bit_size),
			None => Err(LocksidianError::new(opts::usage()))
		}
	}
    else if matches.opt_present("identity-import") {
        match matches.opt_str("identity-import") {
            Some(path) => identity_cli::import_identity_from_pem_file(path),
            None => Err(LocksidianError::new(opts::usage()))
        }
    }
    else if matches.opt_present("identity-export") {
        match matches.opt_str("identity-export") {
            Some(hash) => identity_cli::export_identity(hash),
            None => Err(LocksidianError::new(opts::usage()))
        }
    }
	// Unknown option
    else {
        Err(LocksidianError::new(opts::usage()))
    }
}