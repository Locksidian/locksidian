use sqlite;
use diesel::prelude::*;

use persistence;
use persistence::repository::*;

table! {
    posts {
        id -> Integer,
        title -> VarChar,
        body -> VarChar,
    }
}

#[derive(Queryable, Insertable)]
#[table_name="posts"]
struct Post {
    id: i32,
    title: String,
    body: String
}

struct PostRepository {
    connection: sqlite::Connection
}

impl PostRepository {
    fn new(connection: sqlite::Connection) -> PostRepository {
        PostRepository {
            connection: connection
        }
    }
}

/*impl RepositoryMetadata for PostRepository {
    fn setup_table(&self) -> sqlite::Result<()> {
        self.connection.execute(
            r#"CREATE TABLE IF NOT EXISTS post(
                "id" INTEGER PRIMARY KEY NOT NULL,
                "title" TEXT DEFAULT '',
                "body" TEXT DEFAULT ''
            );
            CREATE UNIQUE INDEX post_id_uindex ON post ("id");"#
        )
    }

    fn drop_table(&self) -> sqlite::Result<()> {
        self.connection.execute("DROP TABLE post;")
    }
}

impl QueryRepository<Post, i32> for PostRepository {
    fn get(&self, pk: i32) -> Option<Post> {
        let post = posts::table.filter(posts::id.eq(pk)).limit(1).load::<Post>(&self.connection);

        match post {
            Ok(post) => post.get(0),
            Err(err) => None
        }
    }

    fn get_all(&self) -> Option<Vec<Post>> {
        unimplemented!()
    }
}*/