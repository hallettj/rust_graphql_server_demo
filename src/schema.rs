use async_graphql::{ComplexObject, Object, SimpleObject};

pub struct Query;

#[Object]
impl Query {
    /// Get every post
    async fn posts(&self) -> Vec<Post> {
        vec![Post {
            id: 1,
            author_id: 1,
            content: Some("Hello, world!".to_string()),
        }]
    }
}

#[derive(Debug, SimpleObject)]
pub struct User {
    id: i32,
    username: String,
}

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct Post {
    id: i32,
    content: Option<String>,

    #[graphql(skip)]
    author_id: i32,
}

#[ComplexObject]
impl Post {
    async fn author(&self) -> User {
        User {
            id: self.author_id,
            username: "mock user".to_string(),
        }
    }
}
