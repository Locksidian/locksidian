//! Identity management.

// TODO: remove after development phase
#![allow(dead_code)]
#![allow(unused_variables)]

use sec::sha::*;
use sec::ripemd::*;
use sec::rsa::Rsa;
use sec::hex::ToHex;

pub mod repository;

pub struct Identity {
    hash: String,
    key: Rsa
}

impl Identity {

    /// Instantiate a new `Indentity` by providing an existing key.
    fn new(key: Rsa) -> Result<Identity, String> {
        Ok(Identity {
            hash: Identity::compute_key_hash(&key)?,
            key: key
        })
    }
    
    /// Instantiate a new `Identity` by generating a new `RSA` keypair.
    fn generate(key_size: u32) -> Result<Identity, String> {
        let keypair = Rsa::generate(key_size)?;
        
        Identity::new(keypair)
    }
	
	/// Compute the public key hash.
	fn compute_key_hash(key: &Rsa) -> Result<String, String> {
		let sha_hash = sha512(key.export_public_key()?.as_slice());
		let hash = ripemd160(sha_hash.as_bytes());
		
		Ok(hash)
	}
	
	/// Export the `Identity`'s PEM-encoded private key to an hexadecimal string.
	fn private_key_to_hex(&self) -> Result<String, String> {
		let private_pem = self.key.export_private_key()?;
		let private_hex = private_pem.to_hex();
		
		Ok(private_hex)
	}
	
	/// Export the `Identity`'s PEM-encoded public key to an hexadecimal string.
	fn public_key_to_hex(&self) -> Result<String, String> {
		let public_pem = self.key.export_public_key()?;
		let public_hex = public_pem.to_hex();
		
		Ok(public_hex)
	}
	
	/// Hash getter.
	fn hash(&self) -> String {
		self.hash.clone()
	}
	
	/// Key getter.
	fn key(&self) -> &Rsa {
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