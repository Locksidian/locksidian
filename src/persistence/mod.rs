//! Data persistence module.

mod repository;

use sqlite::*;

const DATABASE_PATH: &'static str = "dat";

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