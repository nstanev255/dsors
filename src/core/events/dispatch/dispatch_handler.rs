use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::core::events::dispatch::on_ready::on_ready;
use crate::core::events::event::Event;
use crate::core::websocket::WsConnection;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum DispatchType {
    READY,
    GUILD_CREATE,
}

#[derive(Deserialize)]
pub struct DispatchEvent {
    t: DispatchType
}

#[async_trait]
impl Event for DispatchEvent {
    async fn handle(&self, ws_connection: &mut WsConnection, req_message: String) {
        match self.t {
            DispatchType::READY => {
                // On Ready
                on_ready(ws_connection, req_message).await;
            }
            DispatchType::GUILD_CREATE => {
                // On Guild Create
            }
        }
    }
}