//! Options module.
//!
//! Interact with the startup arguments using `getopts` and the environment variables.

use std::env;
use getopts::{Options, Matches};

fn build_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "display this help menu");
    opts.optflag("v", "version", "output version information and exit");

    opts
}

pub fn init() -> Matches {
    let args: Vec<String> = env::args().map(|arg| arg.to_string()).collect();

    match build_opts().parse(&args[1..]) {
        Ok(matcher) => matcher,
        Err(err) => panic!(err.to_string())
    }
}

pub fn env(env_name: &'static str) -> Option<String> {
    match env::var(env_name) {
        Ok(config_result) => Some(config_result),
        Err(_) => None
    }
}

pub fn version() -> String {
    format!(
        "{} v{}",
        ::PACKAGE, ::VERSION
    )
}

pub fn usage() -> String {
    let brief = format!(
        "{}: \"{}\" ({})\nUsage: {} [options]",
        version(), ::DESCRIPTION, ::AUTHORS, ::PACKAGE
    );

    build_opts().usage(&brief)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn opts_init_should_not_panic() {
        init();
    }
}