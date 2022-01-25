use async_graphql::{ComplexObject, Context, Result, SimpleObject};

use crate::db::get_db;

use super::post::Post;

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[ComplexObject]
impl User {
    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let posts = sqlx::query_as!(
            Post,
            "
            select id, author_id, content from posts
            where author_id = $1
            ",
            self.id,
        )
        .fetch_all(get_db(ctx)?)
        .await?;
        Ok(posts)
    }
}
