use std::net::TcpStream;
use serde::Deserialize;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;
use crate::core::events::event::{Event, Opcode};

#[derive(Deserialize)]
pub struct HelloEvent {
    op: Opcode,
    d: Option<HelloData>
}

#[derive(Deserialize)]
struct HelloData {
    heartbeat_interval: i32
}

impl Event for HelloEvent {
    fn handle(&self, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
        println!("event here...")
        // TODO: Handle...
    }
}