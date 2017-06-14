//! Block domain structure.

use error::*;

use num::pow::checked_pow as pow;
use num_bigint::{BigUint, ToBigUint};

use sec::sha::sha512;
use sec::hex::*;

use blockchain::get_current_timestamp;
use blockchain::algorithm::ProofOfWork;
use blockchain::identity::Identity;

use super::*;

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
	pub fn new(data: String, author: &Identity, repository: &BlockRepository) -> LocksidianResult<Self> {
		// Block creation timestamp
		let timestamp = get_current_timestamp();
		let received_at = get_current_timestamp();
		
		// Compute data hash and browse the blockchain in order to find a possible duplicate
		let data_hash = sha512(data.as_bytes());
		Block::assert_document_uniqueness(data_hash.as_ref(), &repository)?;
		
		// Create a partial `Block` structure used to calculate the PoW algorithm
		let signature = author.key().sign(data.as_bytes())?;
		let head = repository.get_head().unwrap_or(BlockEntity::empty());
		
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
			author: author.hash(),
			received_at: received_at,
			received_from: author.hash()
		};

		// Compute the PoW
		let (hash, nonce) = block.compute()?;
		block.nonce = nonce;
		block.hash = hash;

		// Return our complete `Block` structure
		Ok(block)
	}

	/// Adapt a `BlockEntity` into a `Block` structure, consuming its instance.
	pub fn from_entity(entity: BlockEntity) -> LocksidianResult<Self> {
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
			Err(err) => Err(LocksidianError::from_err(err))
		}
	}
	
	/// Create a new `Block` structure from the replication data provided by one of the network peers
	/// through the use of a `BlockReplicationDto`.
	pub fn replicate_from(dto: BlockReplicationDto, repository: &BlockRepository) -> LocksidianResult<Self> {
		let replica = Block::partial_replica(dto)?;
		let data_hash = Block::check_data_hash(&replica)?;
		
		Block::assert_document_uniqueness(data_hash.as_ref(), &repository)?;
		replica.validate()?;
		
		Ok(replica)
	}
	
	/// Create a partial `Block` replica from a `BlockReplicationDto`.
	fn partial_replica(dto: BlockReplicationDto) -> LocksidianResult<Self> {
		match dto.signature.from_hex() {
			Ok(signature) => Ok(Block {
				data: dto.data,
				
				data_hash: dto.data_hash,
				signature: signature,
				timestamp: dto.timestamp,
				nonce: dto.nonce,
				previous: dto.previous,
				
				hash: dto.hash,
				height: dto.height,
				next: String::new(),
				author: dto.author,
				received_at: get_current_timestamp(),
				received_from: dto.received_from
			}),
			Err(err) => Err(LocksidianError::from_err(err))
		}
	}
	
	/// Returns an error if the recomputed data checksum does not match the stored `data_hash`.
	fn check_data_hash(block: &Block) -> LocksidianResult<String> {
		let recomputed_data_hash = sha512(block.data.as_bytes());
		
		match block.data_hash == recomputed_data_hash {
			true => Ok(recomputed_data_hash),
			false => Err(LocksidianError::new(String::from("Replica data_hash does not match the recomputed data checksum")))
		}
	}
	
	/// Returns an `Error` if the specified document hash does exist in the local registry.
	fn assert_document_uniqueness(data_hash: &str, repository: &BlockRepository) -> LocksidianResult<()> {
		match repository.get_by_data_hash(data_hash) {
			Some(entity) => Err(LocksidianError::new(
				format!("Document hash {} is already stored in block {}", data_hash, entity.hash)
			)),
			None => Ok(())
		}
	}
	
	/// Calculate the current `Block` hash.
	fn calculate_hash(&self) -> String {
		let pow_buffer = format!("{}{}{}{}{}", self.data_hash, self.signature.to_hex(), self.timestamp(), self.nonce, self.previous);
		sha512(pow_buffer.as_bytes())
	}
	
	/// If the provided `pow_value` (representing the decimal value of `pow_hash`) is lower than the
	/// Proof of Work `target`, a tuple containing the PoW hash and nonce is returned.
	fn is_pow_valid(&self, pow_hash: String, pow_value: BigUint, target: &BigUint) -> Option<(String, u32)> {
		match pow_value < *target {
			true => Some((pow_hash, self.nonce)),
			false => None
		}
	}
	
	/// Validate the PoW computation for the current `Block` instance.
	fn validate_with_target(&self, target: &BigUint) -> LocksidianResult<Option<(String, u32)>> {
		let hash = self.calculate_hash();
		
		match BigUint::parse_bytes(hash.as_bytes(), 16) {
			Some(pow_value) => Ok(self.is_pow_valid(hash, pow_value, target)),
			None => return Err(LocksidianError::new(format!("Unable to compute block's PoW: {} could not be converted to BigUint", hash)))
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

impl ProofOfWork for Block {

	/// Calculate the Proof of Work difficulty for the given `Block`.
	fn difficulty(&self) -> LocksidianResult<usize> {
		let base = 512;
		let divider = 32;

		let difficulty = base - self.data().len() / divider;

		Ok(difficulty)
	}

	/// Calculate the Proof of Work target based on the given `difficulty` factor.
	fn target(&self, difficulty: usize) -> LocksidianResult<BigUint> {
		let base = 2;

		match base.to_biguint() {
			Some(base) => match pow(base, difficulty) {
				Some(target) => Ok(target),
				None => Err(LocksidianError::new(format!("Unable to compute block's PoW: could not calculate 2^{}", difficulty)))
			},
			None => Err(LocksidianError::new(format!("Unable to compute block's PoW: {} could not be converted to BigUint", base)))
		}
	}

	/// Compute the `Block` nonce using the proof of work algorithm.
	fn compute(&mut self) -> LocksidianResult<(String, u32)> {
		let difficulty = self.difficulty()?;
		let target = self.target(difficulty)?;
		self.signature = self.signature().to_vec();

		self.nonce = 0;

		loop {
			match self.validate_with_target(&target) {
				Ok(Some(result)) => return Ok(result),
				Ok(None) => { self.nonce += 1; },
				Err(err) => return Err(LocksidianError::from_err(err))
			}
		};
	}

	fn validate(&self) -> LocksidianResult<Option<(String, u32)>> {
		let difficulty = self.difficulty()?;
		let target = self.target(difficulty)?;

		self.validate_with_target(&target)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	fn mock_block_data(data: &str) -> Block {
		Block {
            data: String::from(data),

            data_hash: String::new(),
            signature: vec![],
            timestamp: 0,
            nonce: 0,
            previous: String::new(),

            hash: String::new(),
            height: 0,
            next: String::new(),
            author: String::new(),
            received_at: 0,
            received_from: String::new()
        }
	}

	#[test]
	fn difficulty_should_be_equal_to_512() {
		let block = mock_block_data(r#"{"Hello": "World!"}"#);
		let difficulty = block.difficulty().unwrap();

		assert_eq!(512, difficulty);
	}

	#[test]
	fn difficulty_should_be_equal_to_508() {
		let block = mock_block_data(r#"{"message": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}"#);
		let difficulty = block.difficulty().unwrap();

		assert_eq!(508, difficulty);
	}

	#[test]
	fn difficulty_should_be_equal_to_498() {
		let block = mock_block_data(r#"{"message": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}"#);
		let difficulty = block.difficulty().unwrap();

		assert_eq!(498, difficulty);
	}

	#[test]
	fn should_compute_the_target_for_a_difficulty_of_512() {
		let block = mock_block_data(r#"{"Hello": "World!"}"#);
		let difficulty = block.difficulty().unwrap();
		let target = block.target(difficulty).unwrap();

		assert_eq!("100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", format!("{:x}", target));
	}

	#[test]
	fn should_compute_the_target_for_a_difficulty_of_498() {
		let block = mock_block_data(r#"{"message": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}"#);
		let difficulty = block.difficulty().unwrap();
		let target = block.target(difficulty).unwrap();

		assert_eq!("40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", format!("{:x}", target));
	}

	#[test]
	fn block_pow_should_compute_a_nonce_of_0() {
		let mut block = mock_block_data(r#"{"Hello": "World!"}"#);
		let (hash, nonce) = block.compute().unwrap();

		assert_eq!(0, nonce);
		assert_eq!("8ab3361c051a97ddc3c665d29f2762f8ac4240d08995f8724b6d07d8cbedd32c28f589ccdae514f20a6c8eea6f755408dd3dd6837d66932ca2352eaeab594427", hash);
	}

	#[test]
	fn block_pow_should_compute_a_nonce_of_12623() {
		let mut block = mock_block_data(r#"{"message": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}"#);
		let (hash, nonce) = block.compute().unwrap();

		assert_eq!(12623, nonce);
		assert_eq!("0001357cc00eaa17d81b9026372bc291fde84b7936fc8870534efbcf30f0c808b4fa1b94831b955293759dd7d9ac3166590fecefa1b0d87ad4fda9a1b45e165e", hash);
	}

	#[test]
	fn block_pow_should_validate_when_target_is_valid() {
		let mut block = mock_block_data(r#"{"message": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}"#);
		block.nonce = 12623;

		let result = block.validate().unwrap();
		assert_eq!(Some((block.calculate_hash(), block.nonce)), result);
	}

	#[test]
	fn block_pow_should_not_validate_when_nonce_is_not_ok() {
		let mut block = mock_block_data(r#"{"message": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}"#);
		block.nonce = 12622;

		let result = block.validate().unwrap();
		assert_eq!(None, result);
	}
}