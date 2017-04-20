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
	
	pub fn new(identity: &Identity) -> Result<IdentityEntity, String> {
		Ok(IdentityEntity {
			hash: identity.hash(),
			keypair: identity.private_key_to_hex()?,
			active: false
		})
	}
	
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
	
	pub fn set_active(&mut self, active: bool) {
		self.active = active;
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
	
	/// Returns the currently active `IdentityEntity`, or `None` if none is active.
	pub fn get_active(&self) -> Option<IdentityEntity> {
		match identities::table.filter(identities::active.eq(true)).first(self.connection) {
			Ok(entity) => Some(entity),
			Err(_) => None
		}
	}
}

crud_repository!(identities, IdentityEntity, String, hash, IdentityRepository<'pool>);