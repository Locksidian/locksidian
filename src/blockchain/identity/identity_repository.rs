//! Identity Repository.

use persistence::prelude::*;
use blockchain::identity::Identity;

use sec::rsa::Rsa;
use sec::hex::FromHex;

table! {
	identities(hash) {
		hash -> VarChar,
		keypair -> VarChar,
		active -> Bool,
	}
}

#[derive(
	Debug, Clone,
	Queryable, Insertable, AsChangeset
)]
#[table_name = "identities"]
pub struct IdentityEntity {
	hash: String,
	keypair: String,
	active: bool
}

impl IdentityEntity {
	
	/// Instantiate a new `IdentityEntity` based on the given `Identity`.
	///
	/// The private key is PEM-encoded and stored as an hexadecimal string.
	pub fn new(identity: &Identity) -> Result<IdentityEntity, String> {
		Ok(IdentityEntity {
			hash: identity.hash(),
			keypair: identity.private_key_to_hex()?,
			active: false
		})
	}
	
	/// This `IdentityEntity` is adapted to an `Identity` structure.
	///
	/// If the `hash` generated from the new `Identity` structure differs from the one stored
	/// into this `IdentityEntity`, the `RSA` keypair may have be corrupted, resulting in an `Err`.
	pub fn to_identity(&self) -> Result<Identity, String> {
		match self.keypair.from_hex() {
			Ok(key_pem) => {
				let keypair = Rsa::from_private_key(key_pem.as_slice(), "")?;
				let identity = Identity::new(keypair)?;
				
				if self.hash == identity.hash() {
					Ok(identity)
				}
				else {
					Err(String::from("Identity hash mismatch!"))
				}
			},
			Err(err) => Err(err.to_string())
		}
	}
	
	/// `active` setter.
	pub fn set_active(&mut self, active: bool) {
		self.active = active;
	}

	/// `keypair` getter.
	pub fn keypair(&self) -> String {
		self.keypair.clone()
	}
}

pub struct IdentityRepository<'pool> {
	connection: &'pool SqliteConnection
}

impl<'pool> IdentityRepository<'pool> {
	
	/// Instantiate a new `IdentityRepository` whose lifetime is bound to its pooled connection.
	pub fn new(connection: &SqliteConnection) -> IdentityRepository {
		IdentityRepository {
			connection: connection
		}
	}
	
	/// Return the currently active `IdentityEntity`, or `None` if none is active.
	pub fn get_active(&self) -> Option<IdentityEntity> {
		match identities::table.filter(identities::active.eq(true)).first(self.connection) {
			Ok(entity) => Some(entity),
			Err(_) => None
		}
	}
	
	/// Return all the `IdentityEntity` that are set as active.
	pub fn get_actives(&self) -> Option<Vec<IdentityEntity>> {
		match identities::table.filter(identities::active.eq(true)).load(self.connection) {
			Ok(entities) => Some(entities),
			Err(_) => None
		}
	}
	
	/// Gather all the active `IdentityEntity` that are persisted, set them as inactive and update
	/// them. Then, persist the given `IdentityEntity` as the only active one.
	pub fn save_as_active(&self, entity: &mut IdentityEntity) -> Result<usize, String> {
		self.set_all_inactive();
		entity.set_active(true);
		
		self.save(&entity)
	}

	/// Gather all the active `IdentityEntity` that are persisted, set them as inactive and update
	/// them. Then, update the given `IdentityEntity` as the only active one.
	pub fn update_as_active(&self, entity: &mut IdentityEntity) -> Result<usize, String> {
		self.set_all_inactive();
		entity.set_active(true);
		
		self.update(&entity)
	}
	
	/// Update all the active `IdentityEntity` to set them to inactive.
	fn set_all_inactive(&self) {
		match self.get_actives() {
			Some(mut entities) => {
				for mut entity in entities.iter_mut() {
					entity.set_active(false);
					
					match self.update(&entity) {
						Ok(_) => (),
						Err(_) => ()
					}
				}
			}
			None => ()
		}
	}
}

crud_repository!(identities, IdentityEntity, String, hash, IdentityRepository<'pool>);