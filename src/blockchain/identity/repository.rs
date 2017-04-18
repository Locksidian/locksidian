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
	Serialize, Deserialize,
	Queryable, Insertable, AsChangeset
)]
#[table_name = "identities"]
pub struct IdentityVO {
	hash: String,
	keypair: String,
	active: bool
}

impl IdentityVO {
	
	fn from_identity(identity: Identity) -> Result<IdentityVO, String> {
		Ok(IdentityVO {
			hash: identity.hash(),
			keypair: identity.private_key_to_hex()?,
			active: false
		})
	}
	
	fn to_identity(&self) -> Result<Identity, String> {
		match self.keypair.from_hex() {
			Ok(key_pem) => {
				let keypair = Rsa::from_private_key(key_pem.as_slice(), "")?;
				
				Ok(Identity {
					hash: self.hash.clone(),
					key: keypair
				})
			},
			Err(err) => Err(err.to_string())
		}
	}
	
	fn set_active(&mut self, active: bool) {
		self.active = active;
	}
}

pub struct IdentityRepository<'pool> {
	connection: &'pool SqliteConnection
}

impl<'pool> IdentityRepository<'pool> {
	pub fn new(connection: &SqliteConnection) -> IdentityRepository {
		IdentityRepository {
			connection: connection
		}
	}
}

crud_repository!(identities, IdentityVO, String, hash, IdentityRepository<'pool>);