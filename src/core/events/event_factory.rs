use crate::core::commands::opcode::Opcode;
use crate::core::events::event::{Event, EmptyEvent};

pub struct EventFactory;

impl EventFactory {
    pub fn new_event(
        _: Opcode,
        _: &str,
    ) -> Box<dyn Event> {
        Box::new(EmptyEvent {})
    }
}