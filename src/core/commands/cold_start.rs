use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use crate::core::events::event::Opcode;
use crate::core::events::handlers::hello::{HelloEvent, Identify};
use crate::core::websocket::{Credentials};

/**
  The idea for this method is to catch the initial response from the discord api, which is actually the heartbeat interval...
*/
pub async fn get_heartbeat_interval(socket: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> u64 {
    let message =  socket.next().await.unwrap().expect("error reading from socket");
    let hello_event: HelloEvent = serde_json::from_str(message.to_text().unwrap()).unwrap();

    println!("{:?}", hello_event);

    hello_event.d.unwrap().heartbeat_interval
}

pub fn create_identify(credentials: &Credentials) -> Identify {
    // After we get the HelloEvent, we should send back an identity message, with the proper credentials...
    let token = &credentials.token;

    let identify_intent = Identify {
        op: Opcode::Identify,
        d: crate::core::events::handlers::hello::IdentifyData {
            token: token.to_string(),
            // TODO: Figure out what intents are needed here...
            intents: 513,
            properties: crate::core::events::handlers::hello::IdentifyProperties {
                os: String::from("linux"),
                device: String::from("dsors"),
                browser: String::from("dsors")
            }
        }
    };

    identify_intent
}