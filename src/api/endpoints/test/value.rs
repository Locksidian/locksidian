use persistence::prelude::*;

table! {
    values {
        id -> Integer,
        value -> Integer,
    }
}

/// Example structure of persisted data.
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

/// Example repository structure initialized using the `crud_repository!` macro.
pub struct ValueRepository<'pool> {
    connection: &'pool SqliteConnection
}

impl<'pool> ValueRepository<'pool> {
    pub fn new(connection: &SqliteConnection) -> ValueRepository {
        ValueRepository {
            connection: connection
        }
    }
}

crud_repository!(values, Value, i32, id, ValueRepository<'pool>);