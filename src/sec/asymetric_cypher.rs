use std::path::Path;

pub type PrivateKey = [u8];
pub type PublicKey = [u8];
pub type Passphrase = [u8];
pub type Cypher = [u8];
pub type Message = [u8];

pub struct KeyPair {
    private_key: PrivateKey,
    public_key: PublicKey,
}

pub trait AsymetricCypher {
    fn generate_key_pair(&self, message : Message) -> KeyPair;

    fn cypher(&self) -> Cypher;
    fn decypher(&self) -> String;

    fn import_public_key_from_file(&self, path : Path) -> PublicKey;
    fn import_private_key_from_file(&self, path : Path) -> PrivateKey;
    fn import_private_key_from_file_with_passphrase(&self, path : Path, passphrase : Passphrase) -> PrivateKey;
    fn import_keys_from_file(&self, path : Path) -> KeyPair;

    fn export_public_key_to_file(&self, path : Path, public_key : &PublicKey);
    fn export_private_key_to_file(&self, path : Path);
    fn export_keys_to_file(&self, path : Path);

    fn verify_signature(&self) -> bool;

    fn sign(&self, );
}
