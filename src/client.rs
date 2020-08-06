

use super::events::*;

pub fn event_handler<F>(f: F) -> Box<dyn FnMut(Event)>
        where F: FnMut(Event) + 'static
    {
        Box::new(f)
    }

pub struct FlicClient {
    map: Vec<Box<dyn FnMut(Event)>>,
}

impl FlicClient {
    pub fn new() -> FlicClient {
        FlicClient{map: vec![]}
    }
    pub fn register_event_handler(mut self, event: Box<dyn FnMut(Event)>) -> Self {
        self.map.push(event);
        self
    }
}

