//! RSA implementation and wrapper structure.
//!
//! # RSA implementation
//!
//! Defines an RSA algorithm entry point used by the `Locksidian`'s identity management.
//!
//! Usage:
//!
//! ## Instantiate the RSA structure:
//!
//! ```rust
//! // From an existing `Pkey` structure
//! let rsa = Rsa::new(pkey);
//!
//! // From a public key buffer
//! match Rsa::from_public_key(public_buffer) {
//!     Ok(public_key) => ...,
//!     Err(msg) => panic!(msg)
//! }
//!
//! // From a private key buffer
//! match Rsa::from_private_key(private_buffer) {
//!     Ok(private_key) => ...,
//!     Err(msg) => panic!(msg)
//! }
//!
//! // By generating a new keypair
//! match Rsa::generate(4096) {
//!     Ok(private_key) => ...,
//!     Err(msg) => panic!(msg)
//! }
//! ```
//!
//! # Encrypt and decrypt messages:
//!
//! ```rust
//! let message = "Hello World!";
//!
//! let encrypted = rsa.encrypt(message.as_bytes()).unwrap();
//! let decrypted = rsa.decrypt(&encrypted).unwrap();
//! ```
//!
//! # Sign and verify messages:
//!
//! ```rust
//! let message = "Hello World!";
//!
//! let signature = rsa.sign(message.as_bytes()).unwrap();
//! assert!(rsa.verify_signature(message.as_bytes(), &signature).unwrap());
//! ```
//!
//! # Export keys:
//!
//! ```rust
//! // Export the public key as a PEM-encoded bytes vector
//! match rsa.export_public_key() {
//!     Ok(public_pem) => ...,
//!     Err(msg) => panic!(msg)
//! }
//!
//! // Export the private key as a PEM-encoded bytes vector
//! match rsa.export_private_key() {
//!     Ok(private_pem) => ...,
//!     Err(msg) => panic!(msg)
//! }
//! ```

use openssl::rsa;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use openssl::sign::Verifier;
use openssl::rsa::PKCS1_PADDING;
use openssl::hash::MessageDigest;

/// `RSA` operational structure.
///
/// Designed to manage RSA cryptographic algorithm calls.
struct Rsa {
    pkey: PKey
}

impl Rsa {

    /// Instantiate a new `RSA` structure from an existing `PKey`.
    pub fn new(pkey: PKey) -> Rsa {
        Rsa {
            pkey: pkey
        }
    }

    /// Instantiate a new `RSA` structure from a public key buffer.
    pub fn from_public_key(pem_buffer: &[u8]) -> Result<Rsa, String> {
        match PKey::public_key_from_pem(pem_buffer) {
            Ok(pkey) => Ok(Rsa {
                pkey: pkey
            }),
            Err(err) => Err(err.to_string())
        }
    }

    /// Instantiate a new `RSA` structure from a private key buffer.
    ///
    /// Given the private key could be secured with a passphrase, one has to be provided when calling
    /// this function.
    pub fn from_private_key(pem_buffer: &[u8], passphrase_buffer: &str) -> Result<Rsa, String> {
        match PKey::private_key_from_pem_callback(pem_buffer, |passphrase: &mut [u8]| {
            passphrase.copy_from_slice(passphrase_buffer.as_bytes());
            Ok(passphrase_buffer.len())
        }) {
            Ok(pkey) => Ok(Rsa {
                pkey: pkey
            }),
            Err(err) => Err(err.to_string())
        }
    }

    /// Instantiate a new `RSA` structure by generating a new keypair of the given `size`.
    pub fn generate(size: u32) -> Result<Rsa, String> {
        match rsa::Rsa::generate(size) {
            Ok(rsa) => match PKey::from_rsa(rsa) {
                Ok(pkey) => Ok(Rsa {
                    pkey: pkey
                }),
                Err(pkey_err) => Err(pkey_err.to_string())
            },
            Err(rsa_err) => Err(rsa_err.to_string())
        }
    }

    /// Encrypt the provided `message` slice using the RSA public key.
    fn encrypt(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = Vec::new();

        match self.pkey.rsa() {
            Ok(rsa) => match rsa.public_encrypt(message, &mut buffer, PKCS1_PADDING) {
                Ok(_) => Ok(buffer),
                Err(rsa_err) => Err(rsa_err.to_string())
            },
            Err(pkey_err) => Err(pkey_err.to_string())
        }
    }

    /// Decrypt the provided `message` slice using the RSA private key.
    fn decrypt(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = Vec::new();

        match self.pkey.rsa() {
            Ok(rsa) => match rsa.private_decrypt(message, &mut buffer, PKCS1_PADDING) {
                Ok(_) => Ok(buffer),
                Err(rsa_err) => Err(rsa_err.to_string())
            },
            Err(pkey_err) => Err(pkey_err.to_string())
        }
    }

    /// Sign the provided `message` using the RSA private key.
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        match Signer::new(MessageDigest::sha512(), &self.pkey) {
            Ok(mut signer) => match signer.update(message) {
                Ok(_) => match signer.finish() {
                    Ok(signed_message) => Ok(signed_message),
                    Err(finish_err) => Err(finish_err.to_string())
                },
                Err(update_err) => Err(update_err.to_string())
            },
            Err(signer_err) => Err(signer_err.to_string())
        }
    }

    /// Verify that the provided `message` was signed using this RSA keypair.
    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<bool, String> {
        match Verifier::new(MessageDigest::sha512(), &self.pkey) {
            Ok(mut verifier) => match verifier.update(message) {
                Ok(_) => match verifier.finish(signature) {
                    Ok(is_verified) => Ok(is_verified),
                    Err(finish_err) => Err(finish_err.to_string())
                },
                Err(update_err) => Err(update_err.to_string())
            },
            Err(verifier_err) => Err(verifier_err.to_string())
        }
    }

    /// Export the current `RSA` public key to a PEM-encoded bytes vector.
    fn export_public_key(&self) -> Result<Vec<u8>, String> {
        match self.pkey.public_key_to_pem() {
            Ok(pem) => Ok(pem),
            Err(err) => Err(err.to_string())
        }
    }

    /// Export the current `RSA` private key to a PEM-encoded bytes vector.
    fn export_private_key(&self) -> Result<Vec<u8>, String> {
        match self.pkey.private_key_to_pem() {
            Ok(pem) => Ok(pem),
            Err(err) => Err(err.to_string())
        }
    }
}
