//! Data persistence module.
//!
//! Expose the `get_connection` and `setup_database` methods, along with the various `Repository` traits.
//!
//! The special `database_path` method is platform specific and tries to locate the best directory,
//! based on the operating system the node is currently operating on, to place the `.db` file.
//!
//!  - Windows: `%APPDATA%\locksidian\locksidian.db`
//!  - Linux: `~/.locksidian/locksidian.db`
//!  - Other: `./locksidian.db` (relative to the node's working directory)

#[macro_use]
mod macros;
pub mod repository;
pub mod prelude;

use std::path::Path;
use std::fs;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[cfg(target_os = "windows")]
pub fn database_path() -> String {
    match ::opts::env("APPDATA") {
        Some(appdata) => format!("{}\\locksidian\\locksidian.db", appdata),
        None => String::from("locksidian.db")
    }
}

#[cfg(not(any(
    target_os = "windows"
)))]
pub fn database_path() -> String {
    match ::opts::env("HOME") {
        Some(home) => format!("{}/.locksidian/locksidian.db", home),
        None => String::from("locksidian.db")
    }
}

/// Method used to establish a connection to the persistence context of the application, based on
/// SQLite.
pub fn get_connection(database_path: String) -> Result<SqliteConnection, String> {
    check_database_path(database_path.as_ref());

    match SqliteConnection::establish(database_path.as_str()) {
        Ok(connection) => Ok(connection),
        Err(err) => Err(err.to_string())
    }
}

/// Checks that the specified path exists on the file system. If it is not the case, create the
/// parent directory structure.
pub fn check_database_path(path: &Path) {
    if !path.exists() {
        match path.parent() {
            Some(parent) => fs::create_dir_all(parent).unwrap(),
            None => ()
        }
    }
}

/// Execute the setup script at startup in order to initialize the database schemas.
pub fn setup_database(connection: &SqliteConnection) -> Result<(), String> {
    match connection.execute(r#"
        CREATE TABLE IF NOT EXISTS `identities` (
            `hash` TEXT PRIMARY KEY NOT NULL,
            `keypair` BLOB NOT NULL,
            `active` BOOLEAN DEFAULT FALSE
        );
    "#) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}

#[cfg(test)]
mod test {
    use persistence::prelude::*;

    #[test]
    fn should_establish_a_connection() {
        let connection: Result<SqliteConnection, String> = get_connection(String::from("test-persistence.db"));
        assert!(connection.is_ok());
    }

    #[test]
    fn should_setup_the_database_schemas() {
        let connection = get_connection(String::from("test-persistence.db")).expect("Unable to connect to the database");
        let setup = setup_database(&connection);
        assert!(setup.is_ok())
    }
}