use async_trait::async_trait;
use crate::core::websocket::WsConnection;
#[async_trait]
pub trait Event {
    async fn handle(&self, connection: &mut WsConnection, json_message: String);
}

pub struct EmptyEvent;

#[async_trait]
impl Event for EmptyEvent {
    async fn handle(&self, _: &mut WsConnection, _: String) {
        // We will consider this as a placeholder event, and will be deleted eventually...
        println!("Placeholder event...")
    }
}