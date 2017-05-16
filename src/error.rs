//! Locksidian Error module.

#![allow(dead_code)]

use std::fmt;
use std::error::Error;

type LocksidianResult<T> = Result<T, LocksidianError>;

#[derive(Debug)]
pub struct LocksidianError {
    description: String
}

impl LocksidianError {
    pub fn new(description: String) -> Self {
        LocksidianError {
            description: description
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