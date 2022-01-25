use async_graphql::Context;
use color_eyre::Report;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn init_db() -> Result<Pool<Postgres>, Report> {
    let database_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

pub fn get_db<'a>(ctx: &Context<'a>) -> Result<&'a Pool<Postgres>, async_graphql::Error> {
    ctx.data::<Pool<Postgres>>()
}
