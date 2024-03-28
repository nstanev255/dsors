// For now we will allow dead code, as the library is still in its early stages.
// TODO: Remove this when the library is more stable....
#![allow(dead_code)]
#![allow(unused_variables)]


use url::Url;
use dotenv::dotenv;
use crate::core::websocket::WsConnection;
use crate::http::gateway::get_gateway_url;

mod core;
mod http;
mod error;

fn main() {
    // Connect to discord api...
    let url = get_gateway_url().unwrap();

    // Load .env file...
    dotenv().ok();

    // Get the discord token.
    let discord_token = std::env::var("DISCORD_API_TOKEN").expect("env var DISCORD_API_TOKEN is missing");

    let mut connection = match WsConnection::connect(Url::parse(url.as_str()).unwrap(), discord_token) {
        Ok(connection) => { connection }
        Err(error) => { panic!("Fatal error: Could not connect...") }
    };

    // Start the connection...
    connection.start();
}