use std::fmt::Debug;
use std::net::TcpStream;
use url::Url;
use tungstenite::{connect as t_connect, WebSocket};

use serde_json;
use serde::Deserialize;
use serde::Serialize;
use tungstenite::stream::MaybeTlsStream;
use crate::core::events::event::get_opcode;
use crate::core::events::event_factory::EventFactory;
use crate::core::heartbeat_response::create_heartbeat_response;
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


pub struct WsConnection {
    _socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl WsConnection {
    fn connect_ws(url: Url) -> Result<WebSocket<MaybeTlsStream<TcpStream>>, DsorsError> {
        match t_connect(url) {
            Ok((socket, _)) => { Ok(socket) }
            Err(err) => { Err(DsorsError::new("Error initializing connection for websocket...")) }
        }
    }

    /**
    This method will only start the connection, but will not enter into life loop...
     */
    pub fn connect(url: Url) -> Result<WsConnection, DsorsError> {
        let socket = match WsConnection::connect_ws(url) {
            Ok(socket) => { socket }
            Err(err) => { return Err(err) }
        };

        Ok(WsConnection {_socket: socket})
    }

    pub fn start(&mut self) {
        loop {
            let message = match self._socket.read() {
                Ok(message) => { message }
                Err(err) => {
                    // If we get an error while reading the socket, this means that we have a problem...
                    format!("Error: {}...", err);
                    continue;
                }
            };

            println!("message: {:?}", message);

            // We get the opcode, initially so we can build the rest of the error...
            let opcode = match get_opcode(&message) {
                Ok(opcode) => { opcode }
                Err(err) => {
                    println!("Error reading opcode : {}", err);
                    continue;
                }
            };

            println!("opcode {:?}", opcode);
            // Create from factory and call the handle method...
            let event = EventFactory::new_event(opcode, message.to_string().as_str());
            event.handle(self);
        }
    }

    /**
        Send message to the websocket...
    **/
    pub fn send_message<T>(&mut self, data: T) -> Result<(), DsorsError>
        where T: Serialize
    {
        let req_str = serde_json::to_string(&data).unwrap();
        let resp = self._socket.send(tungstenite::Message::Text(req_str));
        if resp.is_err() {
            return Err(DsorsError::new("Error sending message..."));
        }

        Ok(())
    }
}