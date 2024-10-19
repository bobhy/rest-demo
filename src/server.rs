//! The web server itself

use axum::{routing::get, Router};
use tokio;

pub async fn run() {
    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }));

    let addr = "0.0.0.0:3000";
    info!("Running on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
