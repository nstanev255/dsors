// For now we will allow dead code, as the library is still in its early stages.
// TODO: Remove this when the library is more stable....
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::core::websocket::start_loop;

mod core;
mod http;
mod error;

fn main() {
    // Connect to discord api...
    let mut socket = core::initializer::start_connection();

    // Start the lifecycle loop...
    start_loop(&mut socket);
}