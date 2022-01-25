use async_graphql::{Context, Object, Result};

use super::{post::Post, user::User};
use crate::db::get_db;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>, username: String) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            "
            insert into users (username)
            values ($1)
            returning id, username
            ",
            username
        )
        .fetch_one(get_db(ctx)?)
        .await?;
        Ok(user)
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        sqlx::query!("delete from users where id = $1", id)
            .execute(get_db(ctx)?)
            .await?;
        Ok(true)
    }

    async fn create_post(
        &self,
        ctx: &Context<'_>,
        author_id: i32,
        content: String,
    ) -> Result<Post> {
        let post = sqlx::query_as!(
            Post,
            "
            insert into posts (author_id, content)
            values ($1, $2)
            returning id, author_id, content
            ",
            author_id,
            content,
        )
        .fetch_one(get_db(ctx)?)
        .await?;
        Ok(post)
    }

    async fn delete_post(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        sqlx::query!("delete from posts where id = $1", id)
            .execute(get_db(ctx)?)
            .await?;
        Ok(true)
    }
}
