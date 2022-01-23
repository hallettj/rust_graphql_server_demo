mod db;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use color_eyre::Report;
use http::StatusCode;
use std::convert::Infallible;
use tracing::info;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt::format;
use warp::Rejection;
use warp::{http::Response as HttpResponse, Filter};

struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

const PORT: u16 = 8000;

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;
    db::init_db().await?;

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes = graphql_playground
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    info!(port=PORT, url="http://localhost:{PORT}", "Server has started - open the playground in the browser or access the API at http://localhost:8000");

    warp::serve(routes).run(([0, 0, 0, 0], PORT)).await;

    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "full")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .event_format(format().pretty())
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    Ok(())
}
