//! RSA algorithms, used to encrypt and decrypt data

use sec::asymetric_cypher::AsymetricCypher;

/// Process the bytes slice and return its SHA512 hash value.
///
/// Hash size: 512 bits = 64 bytes = 128 chars (hexadecimal string).
///
/// Example usage :
///
/// ```rust
/// use sec::rsa::sha512;
///
/// let hash = sha512("Hello World!".as_bytes());
/// assert_eq!(hash, concat!(
///     "861844d6704e8573fec34d967e20bcfe",
///     "f3d424cf48be04e6dc08f2bd58c72974",
///     "3371015ead891cc3cf1c9d34b49264b5",
///     "10751b1ff9e537937bc46b5d6ff4ecc8"
/// ));
/// ```
///

struct Rsa {
    key_pair : KeyPair
}

impl AsymetricCypher for Rsa {
    fn generate_key_pair(&self, message: Message) -> KeyPair {
        unimplemented!()
    }

    fn cypher(&self) -> Cypher {
        unimplemented!()
    }

    fn decypher(&self) -> String {
        unimplemented!()
    }

    fn import_public_key_from_file(&self, path: Path) -> PublicKey {
        unimplemented!()
    }

    fn import_private_key_from_file(&self, path: Path) -> PrivateKey {
        unimplemented!()
    }

    fn import_private_key_from_file_with_passphrase(&self, path: Path, passphrase: Passphrase) -> PrivateKey {
        unimplemented!()
    }

    fn import_keys_from_file(&self, path: Path) -> KeyPair {
        unimplemented!()
    }

    fn export_public_key_to_file(&self, path: Path, public_key: &PublicKey) {
        unimplemented!()
    }

    fn export_private_key_to_file(&self, path: Path) {
        unimplemented!()
    }

    fn export_keys_to_file(&self, path: Path) {
        unimplemented!()
    }

    fn verify_signature(&self) -> bool {
        unimplemented!()
    }

    fn sign(&self) {
        unimplemented!()
    }
}
