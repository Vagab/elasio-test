mod api;

use api::configuration::{add_api_key, get_api_key};
use api::request::get_weather;
use std::env;

async fn configure(provider: &str) {
    add_api_key(provider);
}

async fn get(provider: &str, address: &str) {
    let api_key = get_api_key(provider);
    get_weather(provider, address, api_key).await;
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "configure" => configure(&args[2]).await,
        "get" => get(&args[2], &args[3]).await,
        _ => panic!("No such command"),
    };
}
