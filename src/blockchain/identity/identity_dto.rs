//! Identity data transfer object.

use blockchain::identity::Identity;

#[derive(
	Debug, Clone,
	Serialize, Deserialize
)]
pub struct IdentityDto {
	hash: String,
	public_key: String
}

impl IdentityDto {
	
	/// Instantiate a new DTO from the `Identity` domain structure.
	pub fn new(identity: Identity) -> Result<IdentityDto, String> {
		Ok(IdentityDto {
			hash: identity.hash(),
			public_key: identity.public_key_to_hex()?,
		})
	}
}