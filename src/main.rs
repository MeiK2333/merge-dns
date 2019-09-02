mod client;
mod config;

fn main() {
    let configs = config::Config::load();
    client::client_handle(configs);
}
