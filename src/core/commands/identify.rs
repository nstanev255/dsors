use serde::Serialize;
use crate::core::commands::opcode::Opcode;
use crate::core::websocket::Credentials;

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


pub fn create_identify(credentials: &Credentials) -> Identify {
    // After we get the HelloEvent, we should send back an identity message, with the proper credentials...
    let token = &credentials.token;

    Identify {
        op: Opcode::Identify,
        d: IdentifyData {
            token: token.to_string(),
            // TODO: Figure out what intents are needed here...
            intents: 513,
            properties: IdentifyProperties {
                os: String::from("linux"),
                device: String::from("dsors"),
                browser: String::from("dsors"),
            },
        },
    }
}