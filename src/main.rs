use url::Url;
use dotenv::dotenv;
use crate::core::websocket::WsConnection;
use crate::http::gateway::get_gateway_url;

mod core;
mod http;
mod error;


#[tokio::main]
async fn main() {
    // Connect to discord api...
    let url = get_gateway_url().await.unwrap();

    // Load .env file...
    dotenv().ok();

    // Get the discord token.
    let discord_token = std::env::var("DISCORD_API_TOKEN").expect("env var DISCORD_API_TOKEN is missing");

    let connection = match WsConnection::connect(Url::parse(url.as_str()).unwrap(), discord_token).await {
        Ok(connection) => { connection }
        Err(_) => { panic!("Fatal error: Could not connect...") }
    };

    // Start the connection...
    WsConnection::start(connection).await;
}