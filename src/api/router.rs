//! API Router

use router::Router;

use super::endpoints;

/// API routes binding.
pub fn routes() -> Router {
    router!(
        // Node API
        index: any "/" => endpoints::node::node_info,

        // Identity API
        identities_all: get "/identities" => endpoints::identities::get_all,
        identities_active: get "/identities/active" => endpoints::identities::get_active_identity,
        identities_hash: get "/identities/:hash" => endpoints::identities::get_identity_by_hash,

        // Block API
        show_head: get "/blocks" => endpoints::blocks::show_head,
        store_document: post "/blocks" => endpoints::blocks::store_document,
        get_block: get "/blocks/:hash" => endpoints::blocks::get_block,
        blocks_replicate: put "/blocks" => endpoints::blocks::replicate_block,

        // Peer API
        register: post "/peers/register" => endpoints::peers::register,
        peers_all: get "/peers" => endpoints::peers::get_all,

        // Metrics API
        metrics: get "/metrics" => endpoints::metrics::get_all,

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