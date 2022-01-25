use async_graphql::{Context, Object, Result};

use super::{post::Post, user::User};
use crate::db::get_db_from_ctx;

pub struct Query;

#[Object]
impl Query {
    /// Get every post
    async fn posts(&self, ctx: &Context<'_>, author_id: Option<i32>) -> Result<Vec<Post>> {
        let posts = sqlx::query_as!(
            Post,
            "
            select id, author_id, content from posts
            where author_id = coalesce($1, author_id)
            ",
            author_id
        )
        .fetch_all(get_db_from_ctx(ctx)?)
        .await?;
        Ok(posts)
    }

    /// Get every user
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let users = sqlx::query_as!(User, "select id, username from users")
            .fetch_all(get_db_from_ctx(ctx)?)
            .await?;
        Ok(users)
    }
}
