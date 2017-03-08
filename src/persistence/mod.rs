//! Data persistence module.
//!
//! Expose the `connect` method and the various `Repository` traits.

mod repository;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[allow(dead_code)]
const DATABASE_PATH: &'static str = "locksidian.db";

/// Method used to establish a connection to the persistence context of the application, based on
/// SQLite.
fn connect(database_url: &'static str) -> Result<SqliteConnection, String> {
    match SqliteConnection::establish(database_url) {
        Ok(connection) => Ok(connection),
        Err(err) => Err(err.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use diesel::sqlite::SqliteConnection;

    #[test]
    fn should_establish_a_connection() {
        let connection: Result<SqliteConnection, String> = connect("test.db");
        assert!(connection.is_ok());
    }
}