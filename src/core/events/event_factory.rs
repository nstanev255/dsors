use crate::core::commands::opcode::Opcode;
use crate::core::events::event::{Event, EmptyEvent};
use crate::core::events::dispatch::dispatch_handler::DispatchEvent;

pub struct EventFactory;

impl EventFactory {
    pub fn new_event(
        opcode: Opcode,
        json_string: &str,
    ) -> Box<dyn Event> {
        match opcode {
            Opcode::Dispatch => {
                let event: DispatchEvent = serde_json::from_str(json_string).unwrap();
                Box::new(event)
            },
            _ => {
                Box::new(EmptyEvent {})
            }
        }
    }
}