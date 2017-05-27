//! Options module.
//!
//! Interact with the startup arguments using `getopts` and the environment variables.

use error::*;

use std::env;
use getopts::{Options, Matches};

fn build_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "display this help menu");
    opts.optflag("v", "version", "output version information and exit");
    
    opts.optopt("d", "daemon", "starts the Locksidian daemon service and HTTP REST API", "LISTEN_ADDR");
    opts.optflag("p", "protected", "starts the Locksidian daemon in protected mode. Only available when running with --daemon");
    
    opts.optopt("i", "identity", "switch the active node identity", "IDENTITY_HASH");
    opts.optopt("", "identity-new", "generate a new identity (defaults to 4096 bit RSA keypair)", "BIT_SIZE");
    opts.optopt("", "identity-import", "import the specified PEM-encoded RSA keypair as the new active identity", "PATH_TO_PEM_FILE");
    opts.optopt("", "identity-export", "export the specified identity keypair to stdout", "IDENTITY_HASH");
    
    opts.optopt("e", "entrypoint", "IP address or hotsname of the network entrypoint", "ADDRESS");

    opts
}

pub fn init() -> LocksidianResult<Matches> {
    let args: Vec<String> = env::args().map(|arg| arg.to_string()).collect();

    match build_opts().parse(&args[1..]) {
        Ok(matcher) => Ok(matcher),
        Err(err) => Err(LocksidianError::from_err(err))
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