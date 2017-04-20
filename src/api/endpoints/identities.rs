//! Identity management endpoint.

use iron::prelude::*;
use persistence::prelude::*;

use blockchain::identity::*;

pub fn get_active_identity(req: &mut Request) -> IronResult<Response> {
	match req.get_connection() {
		Ok(connection) => {
			let repository = IdentityRepository::new(&*connection);
			
			match repository.get_active() {
				Some(entity) => {
					match entity.to_identity() {
						Ok(identity) => {
							match IdentityDto::new(identity) {
								Ok(dto) => response!(Ok, {"identity": dto}),
								Err(msg) => response!(InternalServerError, {"error": msg})
							}
						},
						Err(msg) => response!(InternalServerError, {"error": msg})
					}
				},
				None => response!(NoContent, {"message": "No active identity is currently configured"})
			}
		},
		Err(msg) => response!(InternalServerError, {"error": msg})
	}
}