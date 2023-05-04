use crate::sources::observer::{Event, Publisher};

#[derive(Default)]
pub struct Notifier {
    publisher: Publisher,
    json_data: String,
}

impl Notifier {
    pub fn events(&mut self) -> &mut Publisher {
        &mut self.publisher
    }

    pub fn emit(&mut self, data: String) {
        self.json_data = data.clone();
        self.publisher.notify(Event::Emit, data);
    }
    pub fn save(&self) {
        self.publisher.notify(Event::Emit, self.json_data.clone());
    }
}
