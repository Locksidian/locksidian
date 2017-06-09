//! Hexadecimal conversion from and to byte slices.

use error::*;

use std::fs::File;
use std::io::prelude::*;

pub use rustc_serialize::hex::{FromHex, ToHex};

/// Read the content of an hexadecimal file and convert it to a `Vec<u8>`
pub fn hex_file_to_bytes(path: String) -> LocksidianResult<Vec<u8>> {
	match File::open(path.as_str()) {
		Ok(mut file) => {
			let mut hexadecimal = String::new();
		
			match file.read_to_string(&mut hexadecimal) {
				Ok(_) => match hexadecimal.from_hex() {
					Ok(bytes) => Ok(bytes),
					Err(err) => Err(LocksidianError::from_err(err))
				},
				Err(err) => Err(LocksidianError::from_err(err))
			}
		},
		Err(err) => Err(LocksidianError::from_err(err))
	}
}