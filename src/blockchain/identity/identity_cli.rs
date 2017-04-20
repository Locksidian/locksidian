//! Identity Command Line Interface.

#![allow(dead_code)]

use persistence::prelude::*;
use blockchain::identity::*;

// TODO
fn set_active_identity() {
	unimplemented!()
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

// TODO
fn import_identity_from_pem_file() {
	unimplemented!()
}

// TODO
fn export_identity() {
	unimplemented!()
}