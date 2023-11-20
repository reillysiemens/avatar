use axum::{extract::ConnectInfo, routing::get, Router};
use std::net::SocketAddr;

async fn avatar(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("Hello, {}", addr.ip())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/avatar.png", get(avatar));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let make_service = app.into_make_service_with_connect_info::<SocketAddr>();
    axum::serve(listener, make_service).await.unwrap();
}
