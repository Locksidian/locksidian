//! Command Line Interface.
//!
//! Handle the command line arguments provided by the `opts` module and the environment variable
//! defined in the system/Docker container.

use getopts::Matches;

use opts;
use api;
use persistence::*;

pub fn handle(matches: Matches) {
    if matches.opt_present("help") {
        println!("{}", opts::usage());
    }
    else if matches.opt_present("version") {
        println!("{}", opts::version());
    }
    else if matches.opt_present("daemon") {
        daemon(matches.opt_str("daemon"));
    }
    else if opts::env("LS_DAEMON").is_some() {
        daemon(opts::env("LS_DAEMON"));
    }
    else {
        println!("{}", opts::usage());
    }
}

fn daemon(opt_addr: Option<String>) {
    match opt_addr {
        Some(listen_addr) => {
            match get_connection(DATABASE_PATH) {
                Ok(connection) => {
                    setup_database(&connection).expect("Unable to initialize the database schemas");

                    let server = api::Server::new(listen_addr);
                    server.start(api::router());
                },
                Err(msg) => panic!(msg)
            }


        },
        None => println!("{}", opts::usage())
    }
}