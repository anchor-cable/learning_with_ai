use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower::ServiceBuilder;
// use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

#[derive(Clone)]
struct State {}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // let app = Router::new().route("/", get(hello_world));
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/hello", get(hello_world))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
