use async_graphql::{ComplexObject, Context, Result, SimpleObject};

use super::user::User;
use crate::db::get_db;

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct Post {
    pub id: i32,
    pub content: Option<String>,

    #[graphql(skip)]
    pub author_id: i32,
}

#[ComplexObject]
impl Post {
    async fn author(&self, ctx: &Context<'_>) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            "
            select id, username from users where id = $1
            ",
            self.author_id,
        )
        .fetch_one(get_db(ctx)?)
        .await?;
        Ok(user)
    }
}
