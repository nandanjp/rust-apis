mod config;
mod handlers;
mod models;

use config::Config;
use mongodb::{options::ClientOptions, Client};

#[tokio::main]
async fn main() {
    let config = Config::new();

    let client_options = ClientOptions::parse(config.mongo_config().mongo_uri())
        .await
        .expect("failed to parse the uri: cannot connect to the mongo server");
    let _client = Client::with_options(client_options)
        .expect("failed to get access to the mongo server: ensure to check the uri string");
    println!("Successfully connected to the remote mongo server!")
}
