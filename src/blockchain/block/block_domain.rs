//! Block domain structure.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use sec::sha::sha512;
use sec::hex::ToHex;

use blockchain::identity::Identity;

pub struct Block {
	// Block data
	data: String,
	
	// Block Header
	data_hash: String,
	signature: String,
	timestamp: u64,
	nonce: u32,
	previous: String,
	
	// Block Metadata
	hash: String,
	height: u64,
	next: String,
	author: String,
	received_at: u64,
	received_from: String
}

impl Block {
	
	/// Instantiate a new `Block` containing an arbitrary JSON document.
	pub fn new(data: String, author: &Identity) -> Result<Block, String> {
		let received_at = Block::get_current_timestamp();
		let received_from = author.hash();
		let block_author = author.hash();
		
		let data_hash = sha512(data.as_bytes());
		let signature_bytes = author.key().sign(data.as_bytes())?;
		let signature = signature_bytes.to_hex();
		let timestamp = Block::get_current_timestamp();
		
		Ok(Block {
			data: data,
			
			data_hash: data_hash,
			signature: signature,
			timestamp: timestamp,
			nonce: 0,
			previous: String::from(""),
			
			hash: String::from(""),
			height: 0,
			next: String::from(""),
			author: block_author,
			received_at: 0,
			received_from: received_from
		})
	}
	
	/// Return the current timestamp as an `u64`.
	fn get_current_timestamp() -> u64 {
		let current_time = ::time::get_time();
		let milliseconds = (current_time.sec as u64 * 1000) +
			(current_time.nsec as u64 / 1000 / 1000);
		
		milliseconds
	}
}

#[cfg(test)]
mod test {
	use super::*;
	
	
}