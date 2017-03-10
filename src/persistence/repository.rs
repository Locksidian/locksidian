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

    /// Return the current number of persisted entities of the `T` type.
    fn count(&self) -> Result<i64, String>;
}

/// Repository trait used to command a persisted entity.
///
/// Use this trait to implement the bare command methods for a given entity `T` having the type `U`
/// as its primary key type.
pub trait CommandRepository<T> {

    /// Persist a new `T` entity and return the number of updated rows in the persistence context.
    fn save(&self, entity: &T) -> Result<usize, String>;

    /// Update all the fields of an already persisted `T` entity and return the number of updated rows
    /// in the persistence context.
    fn update(&self, entity: &T) -> Result<usize, String>;

    /// Remove the specified `T` entity from the persistence context and returns `true` if everything
    /// went right, or an error as a `String`.
    fn delete(&self, entity: &T) -> Result<usize, String>;
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
    use diesel;
    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;

    use persistence;
    use persistence::repository::*;

    table! {
        posts {
            id -> Integer,
            title -> VarChar,
            body -> VarChar,
        }
    }

    #[derive(Queryable, Insertable, AsChangeset)]
    #[table_name="posts"]
    struct Post {
        id: i32,
        title: String,
        body: String
    }

    struct PostRepository {
        connection: SqliteConnection
    }

    impl PostRepository {
        fn new(connection: SqliteConnection) -> PostRepository {
            PostRepository {
                connection: connection
            }
        }
    }

    impl RepositoryMetadata for PostRepository {
        fn setup_table(&self) -> Result<(), String> {
            match self.connection.execute(
                r#"CREATE TABLE IF NOT EXISTS posts (
                    "id" INTEGER PRIMARY KEY NOT NULL,
                    "title" TEXT DEFAULT '',
                    "body" TEXT DEFAULT ''
                );"#
            ) {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string())
            }
        }

        fn drop_table(&self) -> Result<(), String> {
            match self.connection.execute("DROP TABLE posts;")  {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string())
            }
        }
    }

    impl QueryRepository<Post, i32> for PostRepository {
        fn get(&self, pk: i32) -> Option<Post> {
            match posts::table.filter(posts::id.eq(pk)).first(&self.connection) {
                Ok(post) => Some(post),
                Err(_) => None
            }
        }

        fn get_all(&self) -> Option<Vec<Post>> {
            match posts::table.load(&self.connection) {
                Ok(posts) => Some(posts),
                Err(_) => None
            }
        }

        fn count(&self) -> Result<i64, String> {
            match posts::table.count().first(&self.connection) {
                Ok(count) => Ok(count),
                Err(err) => Err(err.to_string())
            }
        }
    }

    impl CommandRepository<Post> for PostRepository {
        fn save(&self, entity: &Post) -> Result<usize, String> {
            match diesel::insert(entity).into(posts::table).execute(&self.connection) {
                Ok(inserted_rows) => Ok(inserted_rows),
                Err(err) => Err(err.to_string())
            }
        }

        fn update(&self, entity: &Post) -> Result<usize, String> {
            match diesel::update(posts::table.find(entity.id)).set(entity).execute(&self.connection) {
                Ok(updated_rows) => Ok(updated_rows),
                Err(err) => Err(err.to_string())
            }
        }

        fn delete(&self, entity: &Post) -> Result<usize, String> {
            match diesel::delete(posts::table.filter(posts::id.eq(entity.id))).execute(&self.connection) {
                Ok(deleted_rows) => Ok(deleted_rows),
                Err(err) => Err(err.to_string())
            }
        }
    }

    #[test]
    fn test() {
        const ENTITY_ID: i32 = 1;

        let connection = persistence::connect("test.db").expect("Unable to connect to the database");
        let repository = PostRepository::new(connection);

        repository.setup_table().expect("Unable to create the testing table");

        let opt_posts = repository.get_all();
        assert!(opt_posts.is_some());
        assert_eq!(opt_posts.unwrap().len(), 0);

        let opt_post = repository.get(ENTITY_ID);
        assert!(opt_post.is_none());

        let count = repository.count();
        assert_eq!(count.unwrap(), 0);

        let new_post = Post {
            id: ENTITY_ID,
            title: String::from("Some title"),
            body: String::from("Some body")
        };

        let inserted_rows = repository.save(&new_post);
        assert!(inserted_rows.is_ok());
        assert_eq!(inserted_rows.unwrap(), 1);

        let opt_post = repository.get(ENTITY_ID);
        assert!(opt_post.is_some());

        let count = repository.count();
        assert_eq!(count.unwrap(), 1);

        let mut post = opt_post.unwrap();
        assert_eq!(post.id, ENTITY_ID);
        assert_eq!(post.title, "Some title");
        assert_eq!(post.body, "Some body");

        post.title = String::from("The title");

        let updated_rows = repository.update(&post);
        assert!(updated_rows.is_ok());
        assert_eq!(updated_rows.unwrap(), 1);

        let opt_post = repository.get(ENTITY_ID);
        assert!(opt_post.is_some());

        let post = opt_post.unwrap();
        assert_eq!(post.id, ENTITY_ID);
        assert_eq!(post.title, "The title");
        assert_eq!(post.body, "Some body");

        let deleted_rows = repository.delete(&post);
        assert!(deleted_rows.is_ok());
        assert_eq!(deleted_rows.unwrap(), 1);

        let opt_deleted_post = repository.get(ENTITY_ID);
        assert!(opt_deleted_post.is_none());

        repository.drop_table().expect("Unable to drop the testing table");
    }
}