mod config;
mod server;

fn main() {
    let configs = config::Config::load();
    server::server::server(configs);
}
