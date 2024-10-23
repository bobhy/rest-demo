//! The web server itself

use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use tokio;

mod user;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub addr: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            addr: "0.0.0.0:3001".into(),
        }
    }
}

pub async fn run(settings: &Settings) {
    let mut app = Router::new().route("/", get(|| async { "Hello, Rust!" }));

    app = user::add_routes(app);

    info!("Running on {}", settings.addr);
    let listener = tokio::net::TcpListener::bind(&settings.addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
