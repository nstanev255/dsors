// For now we will allow dead code, as the library is still in its early stages.
// TODO: Remove this when the library is more stable....
#![allow(dead_code)]
#![allow(unused_variables)]

mod core;
mod http;
mod error;

fn main() {
    core::initializer::start_connection();
}