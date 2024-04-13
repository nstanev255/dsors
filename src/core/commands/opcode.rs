use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::error::dsors_error::DsorsError;

#[derive(Debug, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Opcode {
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
pub struct OpcodeEvent {
    op: Opcode,
}

// This function is used to get the initial opcode...
pub fn get_opcode(message: &tokio_tungstenite::tungstenite::protocol::Message) -> Result<Opcode, DsorsError> {
    let json_str = message.to_text().unwrap();

    let event: Result<OpcodeEvent, _> = serde_json::from_str(json_str);
    match event {
        Ok(e) => { Ok(e.op) }
        Err(err) => { Err(DsorsError::new(format!("Error reading to Event the following json string : {}. SerdeError: {}", json_str, err).as_str())) }
    }
}