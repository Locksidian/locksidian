//! Block domain structure.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use sec::sha::sha512;
use sec::hex::FromHex;

use blockchain::identity::Identity;
use blockchain::block::BlockEntity;

pub struct Block {
	// Block data
	data: String,
	
	// Block Header
	data_hash: String,
	signature: Vec<u8>,
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

	/// Adapt a `BlockEntity` into a `Block` structure, consuming its instance.
	pub fn from_entity(entity: BlockEntity) -> Result<Self, String> {
		match entity.signature.from_hex() {
			Ok(signature) => Ok(Block {
				data: entity.data,

				data_hash: entity.data_hash,
				signature: signature,
				timestamp: entity.timestamp as u64,
				nonce: entity.nonce as u32,
				previous: entity.previous,

				hash: entity.hash,
				height: entity.height as u64,
				next: entity.next,
				author: entity.author,
				received_at: entity.received_at as u64,
				received_from: entity.received_from
			}),
			Err(err) => Err(err.to_string())
		}
	}
	
	/// Instantiate a new `Block` containing an arbitrary JSON document.
	pub fn new(data: String, author: &Identity) -> Result<Self, String> {
		let received_at = Block::get_current_timestamp();
		let received_from = author.hash();
		let block_author = author.hash();
		
		let data_hash = sha512(data.as_bytes());
		let signature = author.key().sign(data.as_bytes())?;
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

	/// `data` getter.
	pub fn data(&self) -> String {
		self.data.clone()
	}

	/// `data_hash` getter.
	pub fn data_hash(&self) -> String {
		self.data_hash.clone()
	}

	/// `signature` getter.
	pub fn signature(&self) -> &[u8] {
		self.signature.as_slice()
	}

	/// `timestamp` getter.
	pub fn timestamp(&self) -> u64 {
		self.timestamp
	}

	/// `nonce` getter.
	pub fn nonce(&self) -> u32 {
		self.nonce
	}

	/// `previous` getter.
	pub fn previous(&self) -> String {
		self.previous.clone()
	}

	/// `hash` getter.
	pub fn hash(&self) -> String {
		self.hash.clone()
	}

	/// `height` getter.
	pub fn height(&self) -> u64 {
		self.height
	}

	/// `next` getter.
	pub fn next(&self) -> String {
		self.next.clone()
	}

	/// `author` getter.
	pub fn author(&self) -> String {
		self.author.clone()
	}

	/// `received_at` getter.
	pub fn received_at(&self) -> u64 {
		self.received_at
	}

	/// `received_from` getter.
	pub fn received_from(&self) -> String {
		self.received_from.clone()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	
	
}