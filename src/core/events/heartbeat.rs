use serde::Deserialize;
use crate::core::events::event::{Event, Opcode};
use crate::core::websocket::WsConnection;

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
    fn handle(&self, connection: &WsConnection) {
        //TODO: handle this event...
    }
}