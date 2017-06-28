//! Identity management endpoint.

use iron::prelude::*;
use persistence::prelude::*;

use blockchain::identity::*;

/// Collect all the configured node identities into a single JSON payload of the form:
///
/// ```json
/// [
/// 	{
/// 		"hash": "...",
/// 		"public_key": "..."
/// 	},
/// 	...
/// ]
/// ```
///
/// *Note*: only the **public key** can be transferred through a DTO, in order to avoid the leak
/// of the node's private key.
pub fn get_all(req: &mut Request) -> IronResult<Response> {
	let connection = req.get_connection()?;
	let repository = IdentityRepository::new(&*connection);
	
	match repository.get_all() {
		Some(entities) => {
			let identities: Vec<IdentityDto> = entities.iter()
				.map(|entity| entity.to_identity())
				.filter(|identity| identity.is_ok())
				.map(|identity| identity.unwrap())
				.map(|identity| IdentityDto::new(&identity))
				.filter(|dto| dto.is_ok())
				.map(|dto| dto.unwrap())
				.collect();
			
			http_response!(Ok, identities)
		},
		None => http_response!(NoContent, {})
	}
}

/// Returns only the currently active `Identity` of the node.
///
/// ```json
/// {
/// 	"hash": "...",
/// 	"public_key": "..."
/// }
/// ```
pub fn get_active_identity(req: &mut Request) -> IronResult<Response> {
	let connection = req.get_connection()?;
	
	match identity_cli::get_active_identity(&*connection) {
		Ok(identity) => match IdentityDto::new(&identity) {
			Ok(dto) => http_response!(Ok, dto),
			Err(err) => http_response!(InternalServerError, {"error": err.description()})
		},
		Err(_) => http_response!(NoContent, {})
	}
}

/// Returns the `Identity` identified by the specified `hash`.
///
/// ```json
/// {
/// 	"hash": "{hash}",
/// 	"public_key": "..."
/// }
/// ```
pub fn get_identity_by_hash(req: &mut Request) -> IronResult<Response> {
	match route_param!(req, "hash") {
		Some(hash) => match req.get_connection() {
			Ok(connection) => {
				let repository = IdentityRepository::new(&*connection);
				
				match repository.get(&String::from(hash)) {
					Some(entity) => match entity.to_identity() {
						Ok(identity) => match IdentityDto::new(&identity) {
							Ok(dto) => http_response!(Ok, dto),
							Err(err) => http_response!(InternalServerError, {"error": err.description()})
						},
						Err(err) => http_response!(InternalServerError, {"error": err.description()})
					},
					None => http_response!(NoContent, {})
				}
			},
			Err(err) => http_response!(InternalServerError, {"error": err.description()})
		},
		None => http_response!(BadRequest, {"error": "Hash parameter cannot be empty"})
	}
}