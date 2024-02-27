use std::fmt::Debug;
use url::Url;
use tungstenite::connect as t_connect;

use serde_json;
use serde::Deserialize;
use serde::Serialize;
use crate::ws::heartbeat_response::create_heartbeat_response;

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    op: i8,
    d: Option<HeartbeatMessage>
}

#[derive(Deserialize, Serialize, Debug)]
struct HeartbeatMessage {
    heartbeat_interval: i32
}


pub fn connect() {
    println!("Starting connection...");
    let url = Url::parse("wss://gateway.discord.gg/?v=9&encoding=json").expect("error");
    let (mut socket, _) =
        t_connect(url)
            .expect("error");

    let mut heartbeat_sequence = 0;

    loop {
        // Block on read, for now....
        let message = socket.read();
        if message.is_ok() {
            println!("Connection is established...");
            // If we can read from the api, then this means that we are okay to move forward and send the heartbeat event.
            let message_str = message.unwrap().into_text().unwrap();
            let json_response: Message = serde_json::from_str(message_str.as_str()).unwrap();


            // This means that we need to send the initial heartbeat response...
            if json_response.op == 10 {
                heartbeat_sequence = json_response.d.unwrap().heartbeat_interval;
                let heartbeat_response = serde_json::to_string(&create_heartbeat_response(heartbeat_sequence)).unwrap();
                println!("request {:?}", heartbeat_response);
                socket.send(tungstenite::Message::Text(heartbeat_response));
            } else if json_response.op == 11 {
                // We are inside the heartbeat loop already...
                // This means that we need to send the heartbeat response again.
                let heartbeat_response = serde_json::to_string(&create_heartbeat_response(heartbeat_sequence)).unwrap();
                let res = socket.send(tungstenite::Message::Text(heartbeat_response));
                if !res.is_ok() {
                    // This means that the discord gateway server is down, or something killed our connection...

                }

            }


        }
    }


}