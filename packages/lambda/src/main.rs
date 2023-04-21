#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use lambda_http::{run, service_fn, Body, Error as LambdaError, Request, RequestExt, Response};
use serde_json::json;
use world_id_marbles::Marble;

#[derive(Debug, serde::Serialize)]
struct ErrorResponse {}

impl ErrorResponse {
    fn build(message: &str) -> Response<Body> {
        Response::builder()
            .status(400)
            .header("Content-Type", "application/json")
            .body(Body::Text(
                serde_json::to_string(&json!({ "message": message })).unwrap(),
            ))
            .unwrap()
    }
}

#[allow(clippy::unused_async)]
async fn function_handler(event: Request) -> Result<Response<Body>, LambdaError> {
    let Some(seed) = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("seed")) else {
            return Ok(ErrorResponse::build("Seed not provided."));
        };

    let mut marble = Marble::new(seed);

    let Ok(png) = marble.render_png(1024) else {
        return Ok(ErrorResponse::build("Failed to render marble."));
    };

    let Ok(resp) = Response::builder()
        .status(200)
        .header("content-type", "image/png")
        .body(Body::Binary(png)) else {
            return Ok(ErrorResponse::build("Failed to build response."));
        };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
