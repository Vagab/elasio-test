mod api;

use api::configuration::{add_api_key, get_api_key, get_current_provider, set_current_provider};
use api::request::get_weather;
use std::env;

async fn configure(provider: &str) {
    add_api_key(provider);
}

async fn get(mut provider: &str, address: &str) {
    provider = provider.trim();
    if provider.is_empty() {
        panic!("Something went wrong. Please run `configure`");
    }

    let api_key = get_api_key(provider);
    get_weather(provider, address, api_key).await;
}

async fn get_and_set_current(provider: &str, address: &str) {
    set_current_provider(provider);
    get(provider, address).await;
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "configure" => configure(&args[2]).await,
        "get" => match args.len() {
            3 => get(&get_current_provider(), &args[2]).await,
            4 => get_and_set_current(&args[2], &args[3]).await,
            _ => panic!("Too many arguments!"),
        },
        _ => panic!("No such command"),
    };
}
