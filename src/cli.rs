//! Command Line Interface.
//!
//! Handle the command line arguments provided by the `opts` module and the environment variables
//! defined in the system/Docker container.

use getopts::Matches;
use opts;

use api;
use blockchain::identity;

pub fn handle(matches: Matches) {
	// Generic options
    if matches.opt_present("help") {
        println!("{}", opts::usage());
    }
    else if matches.opt_present("version") {
        println!("{}", opts::version());
    }
	// API
    else if matches.opt_present("daemon") {
        api::cli::start_daemon(matches.opt_str("daemon"));
    }
    else if opts::env("LS_DAEMON").is_some() {
        api::cli::start_daemon(opts::env("LS_DAEMON"));
    }
	// Identitiy
	else if matches.opt_present("identity-new") {
		match matches.opt_str("identity-new") {
			Some(bit_size) => match identity::cli::generate_new_identity(bit_size) {
				Ok(hash) => println!("{}", hash),
				Err(msg) => println!("[-] {}", msg)
			},
			None => println!("{}", opts::usage())
		}
	}
    else if matches.opt_present("identity-export") {
        match matches.opt_str("identity-export") {
            Some(hash) => match identity::cli::export_identity(hash) {
                Ok(private_pem) => println!("{}", private_pem),
                Err(msg) => println!("[-] {}", msg)
            },
            None => println!("{}", opts::usage())
        }
    }
	// Unknown option
    else {
        println!("{}", opts::usage());
    }
}