#[macro_use]
extern crate lazy_static;

mod config;
mod server;

use std::process;

use env_logger;
use log::info;


lazy_static! {
    pub static ref CONFIGS: &'static config::Configs = &config::Configs::load();
}


fn main() {
    env_logger::init();
    info!("server start, pid: {}", process::id());
    server::server::server();
    info!("server ended");
}
