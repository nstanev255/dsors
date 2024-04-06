use serde::{Deserialize, Serialize};
use crate::core::events::event::{Opcode};

#[derive(Deserialize, Debug)]
pub struct HelloEvent {
    op: Opcode,
    pub d: Option<HelloData>
}

#[derive(Deserialize, Debug)]
pub struct HelloData {
    pub heartbeat_interval: u64
}

//TODO: Figure out a way to handle events that we need to send, more clean...
#[derive(Serialize)]
pub struct Identify {
    pub op: Opcode,
    pub d: IdentifyData
}

#[derive(Serialize)]
pub struct IdentifyData {
    pub token: String,
    pub intents: u16,
    pub properties: IdentifyProperties,
}

#[derive(Serialize)]
pub struct IdentifyProperties {
    pub os: String,
    pub browser: String,
    pub device: String
}