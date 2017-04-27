//! Options module.
//!
//! Interact with the startup arguments using `getopts` and the environment variables.

use std::env;
use getopts::{Options, Matches};

fn build_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "display this help menu");
    opts.optflag("v", "version", "output version information and exit");
    
    opts.optopt("d", "daemon", "starts the Locksidian daemon service and HTTP REST API", "LISTEN_ADDR");
    opts.optflag("p", "protected", "starts the Locksidian with protected mode active. Only available when running with --daemon");
    
    opts.optopt("i", "identity", "switch the active node identity", "IDENTITY_HASH");
    opts.optopt("", "identity-new", "generate a new identity (defaults to 4096 bit RSA keypair)", "BIT_SIZE");
    opts.optopt("", "identity-import", "import the specified PEM-encoded RSA keypair as the new active identity", "PATH_TO_PEM_FILE");
    opts.optopt("", "identity-export", "export the specified identity keypair to stdout", "IDENTITY_HASH");

    opts
}

pub fn init() -> Result<Matches, String> {
    let args: Vec<String> = env::args().map(|arg| arg.to_string()).collect();

    match build_opts().parse(&args[1..]) {
        Ok(matcher) => Ok(matcher),
        Err(err) => Err(err.to_string())
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
        "{}: \"{}\"\n{}\n\nUsage: {} [options]",
        version(), ::DESCRIPTION, ::AUTHORS, ::PACKAGE
    );

    build_opts().usage(&brief)
}