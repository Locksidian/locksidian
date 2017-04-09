//! Asymetric Cypher trait
//!
//! # Asymetric Cypher
//!
//! Defines the basics of an asymetric cypher algorithm as supported by Locksidian's blockchain

use std::path::Path;

pub type PrivateKey = Vec<u8>;
pub type PublicKey = Vec<u8>;
pub type Passphrase = [u8];
pub type Cypher = Vec<u8>;
pub type Message = Vec<u8>;
pub type SignedMessage = Vec<u8>;
pub type Signature = [u8];

pub struct KeyPair {
    private_key : PrivateKey,
    public_key : PublicKey
}

/// Trait that enables to define an Asymetric cypher algorithm
/// that will be compatible with Locksidian project
pub trait AsymetricCypher {

    /// Generates a key pair
    /// ```rust
    /// cypher.generate_key_pair(2048);
    /// ```
    fn generate_key_pair(&mut self, size: u32);

    /// Encrypts a message using public key
    /// ```rust
    /// cypher.encrypt_with_public_key(message);
    /// ```
    fn encrypt_with_public_key(&self, message : &Message) -> Cypher;

    /// Encrypts a message using private key
    /// ```rust
    /// cypher.encrypt_with_private_key(message);
    /// ```
    fn encrypt_with_private_key(&self, message : &Message) -> Cypher;

    /// Decrypts a message using public key
    /// ```rust
    /// cypher.decrypt_with_public_key(message);
    /// ```
    fn decrypt_with_public_key(&self, cypher : &Cypher) -> Message;

    /// Decrypts a message using private key
    /// ```rust
    /// cypher.decrypt_with_private_key(message);
    /// ```
    fn decrypt_with_private_key(&self, cypher : &Cypher) -> Message;

    /// Imports public key from PEM file
    /// ```rust
    /// cypher.import_public_key(filepath);
    /// ```
    fn import_public_key(&mut self, path: &Path);

    /// Imports private key from PEM file
    /// ```rust
    /// cypher.import_private_key(filepath);
    /// ```
    fn import_private_key(&mut self, path : &Path);

    /// Imports public key from PEM file with a passphrase
    /// ```rust
    /// cypher.import_public_key(filepath, passphrase);
    /// ```
    fn import_private_key_with_passphrase(&mut self, path : &Path, passphrase : &Passphrase);

    /// Exports public key from PEM file
    /// ```rust
    /// cypher.export_public_key(filepath);
    /// ```
    fn export_public_key(&self, path : &Path);

    /// Exports private key from PEM file
    /// ```rust
    /// cypher.export_private_key(filepath);
    /// ```
    fn export_private_key(&self, path : &Path);

    /// Exports public key from PEM file with a passphrase
    /// ```rust
    /// cypher.export_private_key(filepath, passphrase);
    /// ```
    fn export_private_key_with_passphrase(&self, path: &Path, passphrase: &Passphrase);

    /// Exports key from PEM files
    /// ```rust
    /// cypher.export_keys(filepath);
    /// ```
    fn export_key_pair(&self, path : &Path);

    /// Checks message with given signature
    /// ```rust
    /// cypher.verify_signature(message, signature);
    /// ```
    fn verify_signature(&self, data: &Message, signature: &Signature) -> bool;

    /// Signs a message
    /// ```rust
    /// cypher.sign(message);
    /// ```
    fn sign(&self, data: &Message) -> SignedMessage;
}
