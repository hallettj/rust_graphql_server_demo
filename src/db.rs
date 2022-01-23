use color_eyre::Report;
use sqlx::postgres::PgPoolOptions;

pub async fn init_db() -> Result<(), Report> {
    let database_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(())
}
