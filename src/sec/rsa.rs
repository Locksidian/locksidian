//! RSA algorithms, used to encrypt and decrypt data
extern crate openssl;

use sec::asymetric_cypher::*;
use self::openssl::rsa::Rsa;
use self::openssl::pkey::PKey;
use self::openssl::sign::Signer;
use self::openssl::sign::Verifier;

use std::path::Path;
use self::openssl::hash::MessageDigest;

/// RSA operational structure.
/// Designed to manage RSA cryptographic algorithm calls.
struct RsaService {

    pkey : PKey

}

///
impl RsaService {
    pub fn new(pkey: PKey) -> RsaService {
        RsaService {
            pkey : pkey
        }
    }
}

impl AsymetricCypher for RsaService {
    fn generate_key_pair(&self, size: u32) {
        unimplemented!()
        //self.pkey.rsa().generate(size);
    }
    fn encrypt_with_public_key(&self, message : &Message) -> Cypher {
        unimplemented!()
    }

    fn encrypt_with_private_key(&self, message : &Message) -> Cypher {
        unimplemented!()
    }

    fn decrypt_with_public_key(&self, message : &Message) -> String {
        unimplemented!()
    }

    fn decrypt_with_private_key(&self, message : &Message) -> String {
        unimplemented!()
    }

    fn import_public_key_from_file(&self, path: &Path) -> PublicKey {
        unimplemented!()
    }

    fn import_private_key_from_file(&self, path: &Path) -> PrivateKey {
        unimplemented!()
    }

    fn import_private_key_from_file_with_passphrase(&self, path: &Path, passphrase: Passphrase) -> PrivateKey {
        unimplemented!()
    }

    fn import_keys_from_file(&self, path: &Path) -> KeyPair {
        unimplemented!()
    }

    fn export_public_key_to_file(&self, path: &Path) {
        unimplemented!()
    }

    fn export_private_key_to_file(&self, path: &Path) {
        unimplemented!()
    }

    fn export_keys_to_file(&self, path: &Path) {
        unimplemented!()
    }

    fn verify_signature(&self, data: &Message, signature: &Signature) -> bool {
        let mut verifier = Verifier::new(MessageDigest::sha256(), &self.pkey).unwrap();
        verifier.update(data).unwrap();
        return verifier.finish(&signature).unwrap();
    }

    fn sign(&self, data: &Message) -> SignedMessage {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.pkey).unwrap();
        signer.update(data).unwrap();
        return signer.finish().unwrap();
    }
}
