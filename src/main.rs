mod config;
mod server;

use std::process;

use env_logger;
use log::{info};

fn main() {
    env_logger::init();
    info!("server start, pid: {}", process::id());
    let configs = config::Config::load();
    server::server::server(configs);
    info!("server ended");
}
