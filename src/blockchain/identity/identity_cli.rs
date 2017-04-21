//! Identity Command Line Interface.

use sec::rsa::Rsa;
use sec::hex::*;

use persistence::prelude::*;
use blockchain::identity::*;

/// Return the currently active `Identity`
pub fn get_active_identity(connection: &SqliteConnection) -> Result<Identity, String> {
	let repository = IdentityRepository::new(&connection);
	
	match repository.get_active() {
		Some(entity) => entity.to_identity(),
		None => Err(String::from("Locksidian node cannot operate without an active identity!"))
	}
}

/// Define the `Identity` identified by the provided `hash` as the currently active one.
pub fn set_active_identity(hash: String) -> Result<String, String> {
	let connection = get_connection(database_path())?;
	let repository = IdentityRepository::new(&connection);

	match repository.get(&hash) {
		Some(mut entity) => match repository.update_as_active(&mut entity) {
			Ok(1) => Ok(hash),
			Ok(updated_rows) => Err(format!("An unexpected number of rows were updated in the registry. Expected: 1. Got: {}.", updated_rows)),
			Err(msg) => Err(msg)
		},
		None => Err(format!("An unknown identity hash was provided: {}", hash))
	}
}

/// Generate a new `Identity` and set it as active.
///
/// If the new `Identity` is successfully generated, a `Result` containing its hash is returned:
/// `Ok(hash)`.
///
/// Otherwise, an error is thrown.
pub fn generate_new_identity(requested_key_size: String) -> Result<String, String> {
	match requested_key_size.parse::<u32>() {
		Ok(key_size) => {
			let identity = Identity::generate(key_size)?;
			let mut entity = IdentityEntity::new(&identity)?;
			
			let connection = get_connection(database_path())?;
			let repository = IdentityRepository::new(&connection);
			
			match repository.save_as_active(&mut entity) {
				Ok(1) => Ok(identity.hash()),
				Ok(inserted_rows) => Err(format!("An unexpected number of rows were inserted into the registry. Expected: 1. Got: {}.", inserted_rows)),
				Err(msg) => Err(msg)
			}
		},
		Err(err) => Err(err.to_string())
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
pub fn import_identity_from_pem_file(path: String) -> Result<String, String> {
	let connection = get_connection(database_path())?;
	let repository = IdentityRepository::new(&connection);

	let private_pem = hex_file_to_bytes(path)?;
	let key = Rsa::from_private_key(private_pem.as_slice(), "")?;
	let identity = Identity::new(key)?;

	match repository.get(&identity.hash()) {
		Some(_) => Err(format!("This identity is already configured on this node: {}", identity.hash())),
		None => {
			let entity = IdentityEntity::new(&identity)?;

			match repository.save(&entity) {
				Ok(1) => Ok(identity.hash()),
				Ok(inserted_rows) => Err(format!("An unexpected number of rows were inserted into the registry. Expected: 1. Got: {}.", inserted_rows)),
				Err(msg) => Err(msg)
			}
		}
	}	
}

/// Export the PEM-encoded hexadecimal string representing the private key of the specified
/// `Identity`.
pub fn export_identity(hash: String) -> Result<String, String> {
	let connection = get_connection(database_path())?;
	let repository = IdentityRepository::new(&connection);

	match repository.get(&hash) {
		Some(entity) => Ok(entity.keypair()),
		None => Err(format!("The specified identity does not exists: {}", hash)),
	}
}