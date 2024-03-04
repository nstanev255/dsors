mod ws;
mod json;
mod http;
mod error;

fn main() {
    let gateway_url = ws::initializer::start_connection();
    ws::websocket::connect();
}