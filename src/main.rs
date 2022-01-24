mod api;
mod db;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use color_eyre::Report;
use dotenv;
use http::StatusCode;
use std::convert::Infallible;
use tracing::info;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt::format;
use warp::Rejection;
use warp::{http::Response as HttpResponse, Filter};

use api::{Mutation, Query};

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenv::dotenv()?;
    setup()?;
    let db_pool = db::init_db().await?;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db_pool)
        .finish();

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, Mutation, EmptySubscription>,
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

    let port: u16 = std::env::var("PORT")?.parse()?;
    let url = format!("http://localhost:{port}");

    info!(%port, %url, "Server has started - open the playground in the browser or access the API at the given url");

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    Ok(())
}

fn setup() -> Result<(), Report> {
    color_eyre::install()?;
    tracing_subscriber::fmt::fmt()
        .event_format(format().pretty())
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    Ok(())
}
