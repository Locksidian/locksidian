//! Data persistence module.
//!
//! Expose the `connect` method and the various `Repository` traits.

mod repository;
mod models;

use sqlite::*;

const DATABASE_PATH: &'static str = "locksidian.db";

/// Method used to establish a connection to the persistence context of the application, based on
/// SQLite.
fn connect() -> Result<Connection> {
    open(DATABASE_PATH)
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlite::Connection;

    #[test]
    fn should_return_an_established_connection() {
        let connection: Result<Connection> = connect();
        assert!(connection.is_ok());
    }
}