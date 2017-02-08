//! API Router

use iron::{Response, Request, status};
use router::Router;

/// API routes binding.
pub fn routes() -> Router {
    router!(
        index: get "/" => |_: &mut Request| {
            Ok(Response::with((status::Ok, "Hello World!")))
        }
    )
}


#[cfg(test)]
mod test {
    use api;

    #[test]
    fn routes_are_correctly_bound() {
        api::routes();
    }
}