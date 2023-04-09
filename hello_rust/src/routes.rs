use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use crate::handlers::hello_world;

pub fn app() -> Router {
    Router::new()
        .route("/", get(hello_world::hello_world))
        .route("/hello", get(hello_world::hello_world))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new()))
}