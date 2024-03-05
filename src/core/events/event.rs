use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use tungstenite::Message;
use crate::error::dsors_error::DsorsError;

enum Opcode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11,

}

#[derive(Deserialize)]
pub struct Event {
    op: Opcode
}

impl Event {
    // We will actually generate the event directly from the message that was received from the socket...
    pub fn new(message: Message) -> Result<Event, DsorsError> {
        let json_str = message.to_string().as_str();

        match serde_json::from_str(json_str) {
            Ok(obj) => { Ok(obj) }
            Err(err) => { Err(DsorsError::new(format!("Error reading to Event the following json string : {}", json_str).as_str())) }
        }
    }
}