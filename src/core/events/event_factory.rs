use crate::core::events::event::{Event, Opcode, EmptyEvent};
use crate::core::events::handlers::hello::HelloEvent;

pub struct EventFactory;

impl EventFactory {
    pub fn new_event(
        opcode: Opcode,
        json_string: &str,
    ) -> Box<dyn Event> {
        if opcode == Opcode::Hello {
            let event: HelloEvent = serde_json::from_str(json_string).unwrap();
            Box::new(event)
        } else {
            Box::new(EmptyEvent {})
        }
    }
}