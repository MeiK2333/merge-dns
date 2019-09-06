mod config;
mod server;

use env_logger;
use log::{info};

fn main() {
    env_logger::init();
    info!("server start");
    let configs = config::Config::load();
    server::server::server(configs);
    info!("server ended");
}
