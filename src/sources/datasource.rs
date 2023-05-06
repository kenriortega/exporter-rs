use crate::sources::console::DatasourceConsole;
use crate::sources::kafka::DatasourceKafka;
use crate::sources::postgres::DatasourcePostgres;


pub trait Datasource {
    fn send_data(&self, json: String);
}

pub enum SourceType {
    Kafka,
    Postgresql,
    Stdout,
}

impl SourceType {
    pub fn from_string(source: &str) -> Self {
        let value = match source {
            "Kafka" => SourceType::Kafka,
            "Postgresql" => SourceType::Postgresql,
            _ => SourceType::Stdout,
        };
        value
    }
}

pub struct DatasourceFactory;

impl DatasourceFactory {
    pub fn create_ds(source_type: &SourceType) -> Box<dyn Datasource> {
        match source_type {
            SourceType::Kafka => Box::new(DatasourceKafka),
            SourceType::Postgresql => Box::new(DatasourcePostgres),
            _ => Box::new(DatasourceConsole),
        }
    }
}
