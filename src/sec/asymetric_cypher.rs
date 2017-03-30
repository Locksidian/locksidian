use std::path::Path;

pub type PrivateKey = Vec<u8>;
pub type PublicKey = Vec<u8>;
pub type Passphrase = Vec<u8>;
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
    fn generate_key_pair(&self, size: u32);

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
    fn decrypt_with_public_key(&self, message : &Message) -> String;

    /// Decrypts a message using private key
    /// ```rust
    /// cypher.decrypt_with_private_key(message);
    /// ```
    fn decrypt_with_private_key(&self, message : &Message) -> String;

    /// Imports public key from PEM file
    /// ```rust
    /// cypher.import_public_key_from_file(filepath);
    /// ```
    fn import_public_key_from_file(&self, path : &Path) -> PublicKey;

    /// Imports private key from PEM file
    /// ```rust
    /// cypher.import_private_key_from_file(filepath);
    /// ```
    fn import_private_key_from_file(&self, path : &Path) -> PrivateKey;

    /// Imports public key from PEM file with a passphrase
    /// ```rust
    /// cypher.import_public_key_from_file(filepath, passphrase);
    /// ```
    fn import_private_key_from_file_with_passphrase(&self, path : &Path, passphrase : Passphrase) -> PrivateKey;

    /// Imports key from PEM files
    /// ```rust
    /// cypher.import_keys_from_file(filepath);
    /// ```
    fn import_keys_from_file(&self, path : &Path) -> KeyPair;

    /// Exports public key from PEM file
    /// ```rust
    /// cypher.export_public_key_from_file(filepath);
    /// ```
    fn export_public_key_to_file(&self, path : &Path);

    /// Exports private key from PEM file
    /// ```rust
    /// cypher.export_private_key_from_file(filepath);
    /// ```
    fn export_private_key_to_file(&self, path : &Path);

    /// Exports key from PEM files
    /// ```rust
    /// cypher.export_keys_from_file(filepath);
    /// ```
    fn export_keys_to_file(&self, path : &Path);

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
