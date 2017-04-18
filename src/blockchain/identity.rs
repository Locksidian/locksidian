//! Identity management.

// TODO: remove after development phase
#![allow(dead_code)]
#![allow(unused_variables)]

use sec::sha::*;
use sec::ripemd::*;
use sec::rsa::Rsa;

pub struct Identity {
    hash: String,
    keypair: Rsa
}

impl Identity {

    /// Instantiate a new `Identity` by generating a new `RSA` keypair.
    fn generate(key_size: u32) -> Result<Identity, String> {
        let keypair = Rsa::generate(key_size)?;
        let sha_hash = sha512(keypair.export_public_key()?.as_slice());
        let hash = ripemd160(sha_hash.as_bytes());

        Ok(Identity {
            hash: hash,
            keypair: keypair
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_generate_a_new_identity_from_key_size() {
        let key_size = 2048;

        let identity = Identity::generate(key_size);
        assert!(identity.is_ok());
    }
}