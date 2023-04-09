use std::net::SocketAddr;
mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    let app = routes::app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
