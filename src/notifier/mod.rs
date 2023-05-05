use crate::notifier::notifier::Notifier;
use crate::notifier::observer::Event;
use crate::sources::datasource::SourceType;
use crate::sources::{kafka, postgres};

pub mod notifier;
pub mod observer;


pub fn add_observer(source_type: &SourceType, json_data: String) {
    let mut notifier = Notifier::default();
    match source_type {
        SourceType::Postgresql => {
            notifier.events().subscribe(Event::Emit, kafka::send_data);
            notifier.emit(json_data.clone());
            notifier.events().unsubscribe(Event::Emit, kafka::send_data);
            notifier.save();
        }
        SourceType::Kafka => {
            notifier
                .events()
                .subscribe(Event::Emit, postgres::send_data);
            notifier.emit(json_data.clone());
            notifier
                .events()
                .unsubscribe(Event::Emit, postgres::send_data);
            notifier.save();
        }
    }
}
