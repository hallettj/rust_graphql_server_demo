use async_graphql::{Context, Object, Result};

use super::user::User;
use crate::db::get_db_from_ctx;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>, username: String) -> Result<User> {
        let db = get_db_from_ctx(ctx)?;
        let user = sqlx::query_as!(User, "
            insert into users (username)
            values ($1)
            returning id, username
        ", username).fetch_one(db).await?;
        Ok(user)
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let db = get_db_from_ctx(ctx)?;
        sqlx::query!("
            delete from users where id = $1
        ", id).execute(db).await?;
        Ok(true)
    }
}
