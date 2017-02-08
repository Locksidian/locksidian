//! API Router

use router::Router;

use super::endpoints;

/// API routes binding.
pub fn routes() -> Router {
    router!(
        // Node API
        index: post "/node" => endpoints::node::home,

        // Redirect all other endpoints to the 404 handler
        not_found: get "/**" => endpoints::error::not_found
    )
}

#[cfg(test)]
mod test {
    use api;

    /// Check that the call to api::routes() do **not** panic!().
    #[test]
    fn routes_are_correctly_bound() {
        api::router();
    }
}