use serde::{Deserialize, Serialize};
use crate::core::events::event::{Event, Opcode};
use crate::core::websocket::WsConnection;

#[derive(Deserialize)]
pub struct HelloEvent {
    op: Opcode,
    d: Option<HelloData>
}

#[derive(Deserialize)]
struct HelloData {
    heartbeat_interval: u64
}

#[derive(Serialize)]
struct HelloResponse {
    op: Opcode,
    d: u64,
}

impl Event for HelloEvent {
    fn handle(&self, connection: &mut WsConnection) {
        // Get the data from the interval...
        let interval = match &self.d {
            None => { 0 }
            Some(data) => { data.heartbeat_interval }
        };
        if interval == 0 {
            println!("Error, cannot get interval...")
        }

        println!("{}", interval);

        /* TODO: Handle heartbeat response.
            // Basically we need to send a heartbeat response on every x seconds...
            // Maybe this will be done when we migrate to tokio...
        */
    }
}