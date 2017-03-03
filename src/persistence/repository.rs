//! Command & Query repository traits definition.

/// Repository trait used to query a persisted entity.
///
/// Use this trait to implement the `get` and `get_all` methods for a given entity `T` having the
/// type `U` as its primary key type.
pub trait QueryRepository<T, U> {

    /// Get a single `T` entity using its `U` primary key.
    fn get(&self, pk: U) -> Option<T>;

    /// Get all the persisted `T` entities as a `Vec<T>`.
    fn get_all(&self) -> Option<Vec<T>>;
}

/// Repository trait used to command a persisted entity.
///
/// Use this trait to implement the bare command methods for a given entity `T` having the type `U`
/// as its primary key type.
pub trait CommandRepository<T, U> {

    /// Persist a new `T` entity and return its generated `U` primary key in a `Result` object.
    fn save(&self, entity: &T) -> Result<U, String>;

    /// Update all the fields of an already persisted `T` entity and return the updated entity
    /// wrapped in a `Result` object.
    fn update(&self, entity: &T) -> Result<U, String>;

    /// Remove the specified `T` entity from the persistence context and returns `true` if everything
    /// went right, or an error as a `String`.
    fn delete(&self, entity: &T) -> Result<bool, String>;
}

/// Define the setup and drop scripts for the specified repository.
pub trait RepositoryMetadata {

    /// Execute the setup script for the entity managed by this repository.
    fn setup_table(&self) -> Result<(), String>;

    /// Execute the drop script for the entity managed by this repository.
    fn drop_table(&self) -> Result<(), String>;
}

#[cfg(test)]
mod test {
    use super::*;
    use persistence;
    use sqlite::{Connection, Value};

    struct TestEntity {
        id: i64,
        value: String
    }

    struct TestRepository {
        connection: Connection
    }

    impl TestRepository {
        fn new(connection: Connection) -> TestRepository {
            TestRepository {
                connection: connection
            }
        }
    }

    impl RepositoryMetadata for TestRepository {
        fn setup_table(&self) -> Result<(), String> {
            match self.connection.execute(
                "CREATE TABLE IF NOT EXISTS test_entities (\
                    \"id\" INTEGER PRIMARY KEY NOT NULL,\
                    \"value\" TEXT DEFAULT ''\
                );\
                CREATE UNIQUE INDEX test_entities_id_uindex ON test_entities (\"id\");"
            ) {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string())
            }
        }

        fn drop_table(&self) -> Result<(), String> {
            match self.connection.execute("DROP TABLE test_entities") {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string())
            }
        }
    }

    impl QueryRepository<TestEntity, i64> for TestRepository {
        fn get(&self, pk: i64) -> Option<TestEntity> {
            let mut cursor = self.connection.prepare(
                "SELECT \"id\", \"value\" FROM test_entities WHERE \"id\" = ?"
            ).unwrap().cursor();

            cursor.bind(&[Value::Integer(pk)]).unwrap();

            match cursor.next() {
                Ok(Some(row)) => Some(TestEntity {
                    id: row[0].as_integer().unwrap(),
                    value: String::from(row[1].as_string().unwrap())
                }),
                Ok(None) => None,
                Err(_) => None
            }
        }

        fn get_all(&self) -> Option<Vec<TestEntity>> {
            let mut entities: Vec<TestEntity> = Vec::new();
            let mut cursor = self.connection.prepare(
                "SELECT \"id\", \"value\" FROM test_entities;"
            ).unwrap().cursor();

            while let Some(row) = cursor.next().unwrap() {
                entities.push(TestEntity {
                    id: row[0].as_integer().unwrap(),
                    value: String::from(row[1].as_string().unwrap())
                });
            }

            Some(entities)
        }
    }

    impl CommandRepository<TestEntity, i64> for TestRepository {
        fn save(&self, entity: &TestEntity) -> Result<i64, String> {
            let mut cursor = self.connection.prepare(
                "INSERT INTO test_entities (\"id\", \"value\") VALUES (?, ?)"
            ).unwrap().cursor();

            cursor.bind(&[
                Value::Integer(entity.id),
                Value::String(entity.value.to_string())
            ]).unwrap();

            match cursor.next() {
                Ok(_) => Ok(entity.id),
                Err(err) => Err(err.to_string())
            }
        }

        fn update(&self, entity: &TestEntity) -> Result<i64, String> {
            let mut cursor = self.connection.prepare(
                "UPDATE test_entities SET \"value\" = ? WHERE \"id\" = ?"
            ).unwrap().cursor();

            cursor.bind(&[
                Value::String(entity.value.to_string()),
                Value::Integer(entity.id)
            ]).unwrap();

            match cursor.next() {
                Ok(_) => Ok(entity.id),
                Err(err) => Err(err.to_string())
            }
        }

        fn delete(&self, entity: &TestEntity) -> Result<bool, String> {
            let mut cursor = self.connection.prepare(
                "DELETE FROM test_entities WHERE \"id\" = ?"
            ).unwrap().cursor();

            cursor.bind(&[Value::Integer(entity.id)]).unwrap();

            match cursor.next() {
                Ok(_) => Ok(true),
                Err(err) => Err(err.to_string())
            }
        }
    }

    #[test]
    fn huge_test() {
        const ENTITY_ID: i64 = 1;
        const FIRST_VALUE: &'static str = "First";
        const SECOND_VALUE: &'static str = "Second";

        let connection = persistence::connect().unwrap();
        let repository = TestRepository::new(connection);
        let entity = TestEntity {id: ENTITY_ID, value: String::from(FIRST_VALUE)};

        assert_eq!(repository.setup_table().unwrap(), ());
        assert_eq!(repository.save(&entity).unwrap(), ENTITY_ID);
        assert_eq!(repository.get_all().unwrap().len(), 1);

        let mut persisted_entity = repository.get(ENTITY_ID).unwrap();
        assert_eq!(persisted_entity.id, ENTITY_ID);
        assert_eq!(persisted_entity.value, FIRST_VALUE);

        persisted_entity.value = String::from(SECOND_VALUE);
        assert_eq!(repository.update(&persisted_entity).unwrap(), ENTITY_ID);

        persisted_entity = repository.get(ENTITY_ID).unwrap();
        assert_eq!(persisted_entity.id, ENTITY_ID);
        assert_eq!(persisted_entity.value, SECOND_VALUE);

        assert!(repository.delete(&persisted_entity).is_ok());
        assert_eq!(repository.drop_table().unwrap(), ());
    }
}

