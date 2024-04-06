use crate::core::events::event::{Event, Opcode, EmptyEvent};
pub struct EventFactory;

impl EventFactory {
    pub fn new_event(
        opcode: Opcode,
        json_string: &str,
    ) -> Box<dyn Event> {
        Box::new(EmptyEvent {})
    }
}