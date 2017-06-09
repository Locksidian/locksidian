//! Identity Command Line Interface.

use error::*;

use sec::rsa::Rsa;
use sec::hex::*;
use sec::sha::sha512;
use sec::ripemd::ripemd160;

use persistence::prelude::*;
use blockchain::identity::*;

/// Return the currently active `Identity`
pub fn get_active_identity(connection: &SqliteConnection) -> LocksidianResult<Identity> {
	let repository = IdentityRepository::new(&connection);
	
	match repository.get_active() {
		Some(entity) => entity.to_identity(),
		None => Err(LocksidianError::new(String::from("Locksidian node cannot operate without an active identity!")))
	}
}

/// Define the `Identity` identified by the provided `hash` as the currently active one.
pub fn set_active_identity(hash: String) -> LocksidianResult<String> {
	let connection = get_connection(database_path())?;
	let repository = IdentityRepository::new(&connection);

	match repository.get(&hash) {
		Some(mut entity) => match repository.update_as_active(&mut entity) {
			Ok(1) => Ok(hash),
			Ok(updated_rows) => Err(LocksidianError::new(format!("An unexpected number of rows were updated in the registry. Expected: 1. Got: {}.", updated_rows))),
			Err(err) => Err(err)
		},
		None => Err(LocksidianError::new(format!("An unknown identity hash was provided: {}", hash)))
	}
}

/// Generate a new `Identity` and set it as active.
///
/// If the new `Identity` is successfully generated, a `Result` containing its hash is returned:
/// `Ok(hash)`.
///
/// Otherwise, an error is thrown.
pub fn generate_new_identity(requested_key_size: String) -> LocksidianResult<String> {
	match requested_key_size.parse::<u32>() {
		Ok(key_size) => {
			let identity = Identity::generate(key_size)?;
			let mut entity = IdentityEntity::new(&identity)?;
			
			let connection = get_connection(database_path())?;
			let repository = IdentityRepository::new(&connection);
			
			//TODO: save as inactive
			match repository.save_as_active(&mut entity) {
				Ok(1) => Ok(identity.hash()),
				Ok(inserted_rows) => Err(LocksidianError::new(format!("An unexpected number of rows were inserted into the registry. Expected: 1. Got: {}.", inserted_rows))),
				Err(err) => Err(err)
			}
		},
		Err(err) => Err(LocksidianError::from_err(err))
	}
}

/// Import the content of the specified file as a PEM-encoded hexadecimal string.
///
/// An `RSA` keypair and an `Identity` structure will be created from its content.
///
/// If this `Identity` already exists, an error is thrown. Otherwise, the new identity is stored
/// in the local registry and set as inactive.
///
/// You have to explicitly call `locksidian --identity {hash}` to set an imported `Identity` as active.
pub fn import_identity_from_pem_file(path: String) -> LocksidianResult<String> {
	let connection = get_connection(database_path())?;
	let repository = IdentityRepository::new(&connection);

	let private_pem = hex_file_to_bytes(path)?;
	let key = Rsa::from_private_key(private_pem.as_slice(), "")?;
	let identity = Identity::new(key)?;

	match repository.get(&identity.hash()) {
		Some(_) => Err(LocksidianError::new(format!("This identity is already configured on this node: {}", identity.hash()))),
		None => {
			let entity = IdentityEntity::new(&identity)?;

			match repository.save(&entity) {
				Ok(1) => Ok(identity.hash()),
				Ok(inserted_rows) => Err(LocksidianError::new(format!("An unexpected number of rows were inserted into the registry. Expected: 1. Got: {}.", inserted_rows))),
				Err(err) => Err(err)
			}
		}
	}	
}

/// Export the PEM-encoded hexadecimal string representing the private key of the specified
/// `Identity`.
pub fn export_identity(hash: String) -> LocksidianResult<String> {
	let connection = get_connection(database_path())?;
	let repository = IdentityRepository::new(&connection);

	match repository.get(&hash) {
		Some(entity) => Ok(entity.keypair()),
		None => Err(LocksidianError::new(format!("The specified identity does not exists: {}", hash))),
	}
}

/// Compute the identity hash of the given `Rsa` key.
pub fn compute_key_hash(key: &Rsa) -> LocksidianResult<String> {
	let sha_hash = sha512(key.export_public_key()?.as_slice());
	let hash = ripemd160(sha_hash.as_bytes());
	
	Ok(hash)
}