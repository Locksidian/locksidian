//! RSA implementation
//!
//! # RSA implementation for Locksidian
//!
//! Defines an RSA algorithm entry point as supported by Locksidian's blockchain
//!
//! Usage:
//!
//! # Generate a key pair :
//!
//! ```rust
//! rsa.generate_key_pair(size);
//! ```
//!
//! # Encrypt with public key and decrypt with private one :
//!
//! ```rust
//! cypher = rsa.encrypt_with_public_key(message);
//! message = rsa.decrypt_with_private_key(cypher);
//! ```
//!
//! # Encrypt with private key and decrypt with public one :
//!
//! ```rust
//! cypher = rsa.encrypt_with_private_key(message);
//! message = rsa.decrypt_with_public_key(cypher);
//! ```
//!
//! # Import / Export keys :
//!
//! ```rust
//! rsa.import_public_key(path);
//! rsa.import_public_key(path);
//! rsa.export_public_key(path);
//! rsa.export_public_key(path);
//! rsa.export_key_pair(path);
//! ```
//!
//! # Signatures :
//!
//! ```rust
//! isOk = rsa.verify_signature(message, signature);
//! signedMessage = rsa.sign(message);
//! ```

extern crate openssl;

use sec::asymetric_cypher::*;
use self::openssl::rsa::Rsa;
use self::openssl::pkey::PKey;
use self::openssl::sign::Signer;
use self::openssl::sign::Verifier;
use self::openssl::rsa::Padding;
use self::openssl::symm::Cipher;
use self::openssl::rsa::PKCS1_PADDING;
use std::fs::File;
use std::io::Write;

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
    fn generate_key_pair(&mut self, size: u32) {
        let rsa = Rsa::generate(size).unwrap();
        self.pkey = PKey::from_rsa(rsa).unwrap();
    }

    fn encrypt_with_public_key(&self, message : &Message) -> Cypher {
        let mut cypher : Cypher = Cypher::new();
        self.pkey.rsa().unwrap().public_encrypt(message, &mut cypher, PKCS1_PADDING);
        return cypher;
    }

    fn encrypt_with_private_key(&self, message : &Message) -> Cypher {
        let mut cypher : Cypher = Cypher::new();
        self.pkey.rsa().unwrap().private_encrypt(message, &mut cypher, PKCS1_PADDING);
        return cypher;
    }

    fn decrypt_with_public_key(&self, cypher : &Cypher) -> Message {
        let mut message : Cypher = Cypher::new();
        self.pkey.rsa().unwrap().public_decrypt(cypher, &mut message, PKCS1_PADDING);
        return message;
    }

    fn decrypt_with_private_key(&self, cypher : &Cypher) -> Message {
        let mut message : Cypher = Cypher::new();
        self.pkey.rsa().unwrap().private_decrypt(cypher, &mut message, PKCS1_PADDING);
        return message;
    }

    fn import_public_key(&mut self, path: &Path) {
        let path_string = path.to_str().unwrap().as_bytes();
        self.pkey = PKey::public_key_from_pem(path_string).unwrap();
    }

    fn import_private_key(&mut self, path: &Path) {
        let path_string = path.to_str().unwrap().as_bytes();
        self.pkey = PKey::private_key_from_pem(path_string).unwrap();
    }

    fn import_private_key_with_passphrase(&mut self, path: &Path, passphrase: &Passphrase) {
        let path_string = path.to_str().unwrap().as_bytes();
        self.pkey = PKey::private_key_from_pem_passphrase(path_string, passphrase).unwrap();
    }

    fn export_public_key(&self, path: &Path) {
        let path_string = path.to_str().unwrap();
        let key : Vec<u8> = self.pkey.public_key_to_pem().unwrap();
        let mut output_file = File::create(path_string).unwrap();
        output_file.write_all(key.as_slice());
    }

    fn export_private_key(&self, path: &Path) {
        let path_string = path.to_str().unwrap();
        let key : Vec<u8> = self.pkey.private_key_to_pem().unwrap();
        let mut output_file = File::create(path_string).unwrap();
        output_file.write_all(key.as_slice());
    }

    fn export_private_key_with_passphrase(&self, path: &Path, passphrase: &Passphrase) {
        let path_string = path.to_str().unwrap();
        let key : Vec<u8> = self.pkey.private_key_to_pem_passphrase(Cipher::aes_256_ecb(), passphrase).unwrap();
        let mut output_file = File::create(path_string).unwrap();
        output_file.write_all(key.as_slice());
    }

    fn export_key_pair(&self, path: &Path) {
        let mut public_path_string = path.to_str().unwrap().to_owned();
        public_path_string.push_str(".pub");
        let public_path = Path::new(public_path_string.as_str());
        self.export_private_key(path);
        self.export_public_key(public_path);
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
