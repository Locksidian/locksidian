//! Command Line Interface.
//!
//! Handle the command line arguments provided by the `opts` module and the environment variables
//! defined in the system/Docker container.

use getopts::Matches;

use opts;
use api;

pub fn handle(matches: Matches) {
    if matches.opt_present("help") {
        println!("{}", opts::usage());
    }
    else if matches.opt_present("version") {
        println!("{}", opts::version());
    }
    else if matches.opt_present("daemon") {
        api::cli::start_daemon(matches.opt_str("daemon"));
    }
    else if opts::env("LS_DAEMON").is_some() {
        api::cli::start_daemon(opts::env("LS_DAEMON"));
    }
    else {
        println!("{}", opts::usage());
    }
}