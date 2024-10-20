use pretty_env_logger;
use serde::Deserialize;

#[macro_use]
extern crate log;

use crate::settings::CONFIG;

mod db;
mod roles;
mod server;
mod settings;

/// Main settings
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub level: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            level: "error".into(),
        }
    }
}

#[tokio::main]
async fn main() {
    //todo: feed log level to logger
    pretty_env_logger::init();

    debug!("A debug message from the world");
    info!("A info message from the world");
    trace!("A trace message from the world");
    warn!("A warn message from the world");
    error!("Hello, world!");
    server::run(&CONFIG.server).await;
    info!("shutting down normally.")
}
