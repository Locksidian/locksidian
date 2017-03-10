//! API Router

use router::Router;

use super::endpoints;

/// API routes binding.
pub fn routes() -> Router {
    router!(
        // Node API
        index: any "/" => endpoints::info::node_info,

        // Test endpoints
        test: post "/test" => endpoints::test::home,

        // Redirect all other requests to the 404 handler
        not_found: any "/**" => endpoints::error::not_found
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