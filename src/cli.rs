//! Command Line Interface.
//!
//! Handle the command line arguments provided by the `opts` module and the environment variable
//! defined in the system/Docker container.

use getopts::Matches;
use opts;

pub fn handle(matches: Matches) {
    if matches.opt_present("help") {
        println!("{}", opts::usage());
    }
    else if matches.opt_present("version") {
        println!("{}", opts::version());
    }
    else {
        println!("{}", opts::usage());
    }
}