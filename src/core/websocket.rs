use std::fmt::Debug;
use std::time::Duration;
use url::Url;
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};

use serde_json;
use serde::Deserialize;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use crate::core::commands::cold_start::{create_identify, get_heartbeat_interval};

use crate::core::events::event::{get_opcode, Opcode};
use crate::core::events::event_factory::EventFactory;
use crate::core::events::handlers::hello::HelloEvent;
use crate::error::dsors_error::DsorsError;

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    op: i8,
    d: Option<HeartbeatMessage>,
}

#[derive(Deserialize, Serialize, Debug)]
struct HeartbeatMessage {
    heartbeat_interval: i32,
}

pub struct Credentials {
    pub token: String,
}


pub struct WsConnection {
    _ws_recv: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    _ws_sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>,
    pub credentials: Credentials,
}

impl WsConnection {
    async fn connect_ws(url: Url) -> Result<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, DsorsError> {
        match connect_async(url).await {
            Ok((socket, _)) => { Ok(socket) }
            Err(err) => { Err(DsorsError::new(err.to_string().as_str())) }
        }
    }

    /**
    This method will only start the connection, but will not enter into life loop...
     */
    pub async fn connect(url: Url, token: String) -> Result<WsConnection, DsorsError> {
        let socket = match WsConnection::connect_ws(url).await {
            Ok(socket) => { socket }
            Err(err) => { return Err(err); }
        };

        let (ws_sender, ws_receiver) = socket.split();

        Ok(WsConnection { _ws_recv: ws_receiver, _ws_sender: ws_sender, credentials: Credentials { token } })
    }

    pub async fn authenticate(ws_connection: &mut WsConnection) -> u64 {
        let interval = get_heartbeat_interval(&mut ws_connection._ws_recv).await;
        let identify_intent = create_identify(&ws_connection.credentials);

        interval
    }

    pub async fn start(mut connection: WsConnection) {
        let mut heartbeat_interval = tokio::time::interval(Duration::from_millis(1024));

        loop {
            tokio::select! {
            msg = connection._ws_recv.next() => {
                match msg {
                    Some(msg) => {
                            let msg = msg.unwrap();
                                println!("{:?}", msg);
                            if msg.is_text() ||msg.is_binary() {
                                let opcode = match get_opcode(&msg) {
                                Ok(opcode) => { opcode }
                                Err(err) => {
                                    println!("Error reading opcode : {}", err);
                                    // If we can't read the opcode, this means that we should just exit the program.
                                    std::process::exit(0);
                                 }
                                };
                                println!("opcode {:?}", opcode);
                                // Create from factory and call the handle method...
                                if opcode == Opcode::Hello {
                                    let hello_event: HelloEvent = serde_json::from_str(msg.to_text().unwrap()).unwrap();
                                    println!("Hello event {:?}", hello_event);
                                    let interval = hello_event.d.unwrap().heartbeat_interval;
                                    heartbeat_interval = tokio::time::interval(Duration::from_millis(interval));

                                    // Send authentication
                                    let identity_event = create_identify(&connection.credentials);
                                    connection.send_message(identity_event).await.expect("Cannot send identity command...");

                                } else {
                                    let event = EventFactory::new_event(opcode, msg.to_string().as_str());
                                    event.handle(&mut connection).await;
                                }
                            } else if msg.is_close() {
                                break;
                            }
                    }
                    None => break,
                }
            }
            _ = heartbeat_interval.tick() => {
                    println!("sending heartbeat...")
                    // We will send heartbeat event here, eventually...
            }
        }
        }
    }

    /**
        Send message to the websocket...
    **/
    pub async fn send_message<T>(&mut self, data: T) -> Result<(), DsorsError>
        where T: Serialize
    {
        let req_str = serde_json::to_string(&data).unwrap();
        let resp = self._ws_sender.send(tokio_tungstenite::tungstenite::protocol::Message::Text(req_str)).await;
        if resp.is_err() {
            return Err(DsorsError::new("Error sending message..."));
        }

        Ok(())
    }
}