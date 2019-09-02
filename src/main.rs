mod client;
mod config;
mod server;
mod rr;

fn main() {
    let configs = config::Config::load();
    let _ = server::server(configs);
}
