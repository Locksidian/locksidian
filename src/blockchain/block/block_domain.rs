//! Block domain structure.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use num::pow::checked_pow as pow;
use num_bigint::{BigUint, ToBigUint};

use sec::sha::sha512;
use sec::hex::*;

use blockchain::identity::Identity;
use blockchain::block::{ProofOfWork, BlockEntity, BlockRepository};

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
	
	/// Instantiate a new `Block` containing an arbitrary JSON document.
	pub fn new(data: String, author: &Identity, repository: &BlockRepository) -> Result<Self, String> {
		// Block creation timestamp
		let timestamp = Block::get_current_timestamp();
		let received_at = Block::get_current_timestamp();
		
		// Compute data hash and browse the blockchain in order to find a possible duplicate
		let data_hash = sha512(data.as_bytes());

		match repository.get_by_data_hash(&data_hash) {
			Some(entity) => Err(format!("Document hash {} is already stored in block {}", data_hash, entity.hash)),
			None => {
				let received_from = author.hash();
				let block_author = author.hash();
				let signature = author.key().sign(data.as_bytes())?;

				let head = repository.get_head().unwrap_or(BlockEntity::empty());
				
				// Create a partial `Block` structure used to calculate the PoW algorithm
				let mut block = Block {
					data: data,
					
					data_hash: data_hash,
					signature: signature,
					timestamp: timestamp,
					nonce: 0,
					previous: head.hash,
					
					hash: String::new(),
					height: (head.height + 1) as u64,
					next: String::new(),
					author: block_author,
					received_at: received_at,
					received_from: received_from
				};

				let (hash, nonce) = Block::compute(&block)?;
				block.nonce = nonce;
				block.hash = hash;

				// Return our complete `Block` structure
				Ok(block)
			}
		}
	}
	
	/// Return the current timestamp as an `u64`.
	fn get_current_timestamp() -> u64 {
		let current_time = ::time::get_time();
		let milliseconds = (current_time.sec as u64 * 1000) +
			(current_time.nsec as u64 / 1000 / 1000);
		
		milliseconds
	}

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

impl ProofOfWork<Block> for Block {

	/// Calculate the Proof of Work difficulty for the given `Block`.
	fn difficulty(block: &Block) -> Result<usize, String> {
		let base = 512;
		let divider = 32;

		let difficulty = base - block.data().len() / divider;

		Ok(difficulty)
	}

	/// Compute the `Block` nonce using the proof of work algorithm.
	fn compute(block: &Block) -> Result<(String, u32), String> {
		let base = 2;

		match base.to_biguint() {
			Some(base) => {
				let difficulty = Block::difficulty(&block)?;

				match pow(base, difficulty) {
					Some(pow_target) => {
						let mut nonce = 0;

						let data_hash = block.data_hash();
						let signature = block.signature().to_hex();
						let previous = block.previous();

						loop {
							let pow_buffer = format!("{}{}{}{}{}", data_hash, signature, block.timestamp(), nonce, previous);
							let pow_hash = sha512(pow_buffer.as_bytes());
							
							match BigUint::parse_bytes(pow_hash.as_bytes(), 16) {
								Some(pow_value) => {
									if pow_value < pow_target {
										return Ok((pow_hash, nonce))
									}

									nonce += 1;
								},
								None => return Err(format!("Unable to compute block's PoW: {} could not be converted to BigUint", pow_hash))
							}
						};
					},
					None => Err(format!("Unable to compute block's PoW: could not calculate 2^{}", difficulty))
				}
			},
			None => Err(format!("Unable to compute block's PoW: {} could not be converted to BigUint", base))
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	
	
}