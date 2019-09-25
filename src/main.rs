#[macro_use]
extern crate lazy_static;

mod config;
mod server;

use std::process;

use env_logger;
use log::info;


fn main() {
    env_logger::init();
    info!("server start, pid: {}", process::id());
    server::server::server();
    info!("server ended");
}
