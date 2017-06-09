//! Locksidian Error module.
//!
//! Custom `Result` override and `Error` struct used for error propagation at runtime.

use std::fmt;
pub use std::error::Error;

/// `Result` type override for simplification.
pub type LocksidianResult<T> = Result<T, LocksidianError>;

/// Custom error type used to propagate errors at runtime.
#[derive(Debug)]
pub struct LocksidianError {
    description: String
}

impl LocksidianError {

    /// Instantiate a new `LocksidianError` using a custom description.
    pub fn new(description: String) -> Self {
        LocksidianError {
            description: description
        }
    }

    /// Instantiate a new `LocksidianError` based on the given `Error` cause.
    pub fn from_err<T: Error>(cause: T) -> Self {
        LocksidianError {
            description: cause.description().to_string()
        }
    }
}

impl fmt::Display for LocksidianError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description.as_str())
    }
}

impl Error for LocksidianError {
    fn description(&self) -> &str {
        self.description.as_str()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}