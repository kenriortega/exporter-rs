use crate::sources::{kafka, postgres};
use crate::sources::kafka::DatasourceKafka;
use crate::sources::notifier::Notifier;
use crate::sources::observer::Event;
use crate::sources::postgres::DatasourcePostgres;

pub trait Datasource {
    fn send_data(&self, json: String);
}

pub enum SourceType {
    Kafka,
    Postgresql,
}

pub struct DatasourceFactory;

impl DatasourceFactory {
    pub fn create_ds(source_type: &SourceType) -> Box<dyn Datasource> {
        match source_type {
            SourceType::Kafka => Box::new(DatasourceKafka),
            SourceType::Postgresql => Box::new(DatasourcePostgres),
        }
    }

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
}
