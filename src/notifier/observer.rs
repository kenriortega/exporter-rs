use std::collections::HashMap;

/// An event type.
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Emit,
}

/// A subscriber (listener) has type of a callable function.
pub type Subscriber = fn(json_data: String);

/// Publisher sends events to subscribers (listeners).
#[derive(Default)]
pub struct Publisher {
    events: HashMap<Event, Vec<Subscriber>>,
}

impl Publisher {
    pub fn subscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events
            .get_mut(&event_type)
            .unwrap()
            .retain(|&x| x != listener);
    }

    pub fn notify(&self, event_type: Event, json_data: String) {
        let listeners = self.events.get(&event_type).unwrap();
        for listener in listeners {
            listener(json_data.clone());
        }
    }
}
