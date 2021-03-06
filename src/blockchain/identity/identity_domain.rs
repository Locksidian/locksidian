//! Identity domain structure.

use error::*;

use sec::rsa::Rsa;
use sec::hex::ToHex;

use super::identity_cli::compute_key_hash;

pub struct Identity {
	hash: String,
	key: Rsa
}

impl Identity {
	
	/// Instantiate a new `Indentity` by providing an existing key.
	pub fn new(key: Rsa) -> LocksidianResult<Identity> {
		Ok(Identity {
			hash: compute_key_hash(&key)?,
			key: key
		})
	}
	
	/// Instantiate a new `Identity` by generating a new `RSA` keypair.
	pub fn generate(key_size: u32) -> LocksidianResult<Identity> {
		let keypair = Rsa::generate(key_size)?;
		
		Identity::new(keypair)
	}
	
	/// Export the `Identity`'s PEM-encoded private key to an hexadecimal string.
	pub fn private_key_to_hex(&self) -> LocksidianResult<String> {
		let private_pem = self.key.export_private_key()?;
		let private_hex = private_pem.to_hex();
		
		Ok(private_hex)
	}
	
	/// Export the `Identity`'s PEM-encoded public key to an hexadecimal string.
	pub fn public_key_to_hex(&self) -> LocksidianResult<String> {
		let public_pem = self.key.export_public_key()?;
		let public_hex = public_pem.to_hex();
		
		Ok(public_hex)
	}
	
	/// Hash getter.
	pub fn hash(&self) -> String {
		self.hash.clone()
	}
	
	/// Key getter.
	pub fn key(&self) -> &Rsa {
		&self.key
	}
}

#[cfg(test)]
mod test {
	use super::*;
	
	#[test]
	fn should_create_a_new_identity_from_existing_key() {
		let key = Rsa::generate(2048).unwrap();
		
		let identity = Identity::new(key);
		assert!(identity.is_ok());
	}
	
	#[test]
	fn should_generate_a_new_identity_from_key_size() {
		let key_size = 2048;
		
		let identity = Identity::generate(key_size);
		assert!(identity.is_ok());
	}
}