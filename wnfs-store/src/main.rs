use axum::{
    routing::{get, put},
    Router, Server,
};
use std::net::SocketAddr;
use wnfs_store::{routes, DEFAULT_ADDR, DEFAULT_PORT};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/store", get(routes::get_from_store))
        .route("/store", put(routes::put_in_store));

    let addr = SocketAddr::from((DEFAULT_ADDR, DEFAULT_PORT));

    tracing::debug!("listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
