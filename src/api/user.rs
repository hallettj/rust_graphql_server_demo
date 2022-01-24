use async_graphql::SimpleObject;

#[derive(Debug, SimpleObject)]
pub struct User {
    pub id: i32,
    pub username: String,
}
