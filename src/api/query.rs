use async_graphql::{Context, Object, Result};

use super::post::Post;
use crate::db::get_db_from_ctx;

pub struct Query;

#[Object]
impl Query {
    /// Get every post
    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let db = get_db_from_ctx(ctx)?;
        let posts = sqlx::query_as!(
            Post,
            "
            select id, author_id, content from posts
            "
        )
        .fetch_all(db)
        .await?;
        Ok(posts)
    }
}
