use pretty_env_logger;

#[macro_use]
extern crate log;

mod db;
mod roles;
mod server;
mod settings;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    debug!("A debug message from the world");
    info!("A info message from the world");
    trace!("A trace message from the world");
    warn!("A warn message from the world");
    error!("Hello, world!");
    server::run().await;
    info!("shutting down normally.")
}
