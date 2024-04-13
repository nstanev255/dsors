use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::core::events::event::Event;
use crate::core::websocket::WsConnection;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Type {
    Ready
}

#[derive(Deserialize)]
pub struct DispatchEvent {
    t: Type
}

#[async_trait]
impl Event for DispatchEvent {
    async fn handle(&self, _: &mut WsConnection) {
        println!("{:?}", self.t);
    }
}