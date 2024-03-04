mod ws;
mod json;
mod http;
mod error;

fn main() {
    ws::initializer::start_connection();
}