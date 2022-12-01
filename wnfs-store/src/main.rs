use axum::{
    routing::{get, put},
    Router, Server,
};
use std::net::SocketAddr;
use wnfs_store::{
    routes::{hamt, privateref, store},
    DEFAULT_ADDR, DEFAULT_PORT,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/store", get(store::get_from_store))
        .route("/store", put(store::put_in_store))
        .route("/hamt", get(hamt::get_from_hamt))
        .route("/hamt", put(hamt::put_in_hamt))
        .route("/ref", get(privateref::get_ref))
        .route("/ref", put(privateref::put_ref));

    let addr = SocketAddr::from((DEFAULT_ADDR, DEFAULT_PORT));

    tracing::debug!("listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
