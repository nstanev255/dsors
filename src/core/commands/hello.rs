use serde::Deserialize;
use tokio_tungstenite::tungstenite::Message;
use crate::core::events::event::Opcode;

#[derive(Deserialize, Debug)]
pub struct HelloEvent {
    op: Opcode,
    pub d: Option<HelloData>
}

#[derive(Deserialize, Debug)]
pub struct HelloData {
    pub heartbeat_interval: u64
}

/**
The idea for this method is to catch the initial response from the discord api, which is actually the heartbeat interval...
 */
pub fn get_heartbeat_interval(message: Message) -> u64 {
    let hello_event: HelloEvent = serde_json::from_str(message.to_text().unwrap()).unwrap();
    hello_event.d.unwrap().heartbeat_interval
}