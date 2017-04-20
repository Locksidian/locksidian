//! Persistence macros.
//!
//! # CRUD Repository
//!
//! Automagically implements the `QueryRepository<T, U>` and `CommandRepository<T>` traits to the
//! provided repository structure.
//!
//! Usage:
//!
//! ```rust
//! crud_repository!(entities, Entity, i32, id, EntityRepository);
//!     // Table name ---^        ^      ^   ^        ^
//!        // Entity structure ---|      |   |        |
//!               // Primary key type ---|   |        |
//!                   // Primary key name ---|        |
//!                        // Repository structure ---|
//! ```
//!
//! Example: `crud_repository!(posts, Post, i32, id, PostRepository);`

macro_rules! crud_repository {
    ($table:ident, $entity:ty, $pk:ty, $pk_name:ident, $repository:ty) => {
        impl<'pool> QueryRepository<$entity, $pk> for $repository {
            fn get(&self, pk: &$pk) -> Option<$entity> {
                match $table::table.filter($table::$pk_name.eq(pk)).first(self.connection) {
                    Ok(entity) => Some(entity),
                    Err(_) => None
                }
            }

            fn get_all(&self) -> Option<Vec<$entity>> {
                match $table::table.load(self.connection) {
                    Ok(entities) => Some(entities),
                    Err(_) => None
                }
            }

            fn count(&self) -> Result<i64, String> {
                match $table::table.count().first(self.connection) {
                    Ok(count) => Ok(count),
                    Err(err) => Err(err.to_string())
                }
            }
        }

        impl<'pool> CommandRepository<$entity> for $repository {
            fn save(&self, entity: &$entity) -> Result<usize, String> {
                match ::diesel::insert(entity).into($table::table).execute(self.connection) {
                    Ok(inserted_rows) => Ok(inserted_rows),
                    Err(err) => Err(err.to_string())
                }
            }

            fn update(&self, entity: &$entity) -> Result<usize, String> {
                match ::diesel::update($table::table.find(&entity.$pk_name)).set(entity).execute(self.connection) {
                    Ok(updated_rows) => Ok(updated_rows),
                    Err(err) => Err(err.to_string())
                }
            }

            fn delete(&self, entity: &$entity) -> Result<usize, String> {
                match ::diesel::delete($table::table.filter($table::$pk_name.eq(&entity.$pk_name))).execute(self.connection) {
                    Ok(deleted_rows) => Ok(deleted_rows),
                    Err(err) => Err(err.to_string())
                }
            }
        }
    }
}