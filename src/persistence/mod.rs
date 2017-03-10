//! Data persistence module.
//!
//! Expose the `connect` method and the various `Repository` traits.

mod repository;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[allow(dead_code)]
pub const DATABASE_PATH: &'static str = "locksidian.db";

/// Method used to establish a connection to the persistence context of the application, based on
/// SQLite.
pub fn get_connection(database_url: &'static str) -> Result<SqliteConnection, String> {
    match SqliteConnection::establish(database_url) {
        Ok(connection) => Ok(connection),
        Err(err) => Err(err.to_string())
    }
}

/// Execute the setup script at startup in order to initialize the database schemas.
pub fn setup_database(connection: &SqliteConnection) -> Result<(), String> {
    match connection.execute(r#""#) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use diesel::sqlite::SqliteConnection;

    #[test]
    fn should_establish_a_connection() {
        let connection: Result<SqliteConnection, String> = get_connection("test.db");
        assert!(connection.is_ok());
    }

    #[test]
    fn should_setup_the_database_schemas() {
        let connection = get_connection("test.db").expect("Unable to connect to the database");
        let setup = setup_database(&connection);
        assert!(setup.is_ok())
    }
}