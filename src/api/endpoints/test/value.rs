use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use persistence::prelude::*;

table! {
    values {
        id -> Integer,
        value -> Integer,
    }
}

#[derive(
    Debug, Clone,
    Serialize, Deserialize,
    Queryable, Insertable, AsChangeset
)]
#[table_name = "values"]
pub struct Value {
    id: i32,
    pub value: i32
}

pub struct ValueRepository {
    connection: SqliteConnection
}

impl ValueRepository {
    pub fn new(connection: SqliteConnection) -> ValueRepository {
        ValueRepository {
            connection: connection
        }
    }
}

crud_repository!(values, Value, i32, ValueRepository);

