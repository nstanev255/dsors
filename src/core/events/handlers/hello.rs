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

//TODO: Figure out a way to handle events that we need to send, more clean...
#[derive(Serialize)]
struct Identify {
    op: Opcode,
    d: IdentifyData
}

#[derive(Serialize)]
struct IdentifyData {
    token: String,
    intents: u16,
    properties: IdentifyProperties,
}

#[derive(Serialize)]
struct IdentifyProperties {
    os: String,
    browser: String,
    device: String
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

        // After we get the HelloEvent, we should send back an identity message, with the proper credentials...
        let token = &connection.credentials.token;

        let identify_intent = Identify {
            op: Opcode::Identify,
            d: IdentifyData {
                token: token.to_string(),
                // TODO: Figure out what intents are needed here...
                intents: 513,
                properties: IdentifyProperties {
                    os: String::from("linux"),
                    device: String::from("dsors"),
                    browser: String::from("dsors")
                }
            }
        };

        connection.send_message(identify_intent).expect("Error");

    }
}