use crate::sources::datasource::Datasource;

pub struct DatasourceKafka;

impl Datasource for DatasourceKafka {
    fn send_data(&self, json: String) {
        println!("kafka: {}", json)
    }
}

pub fn send_data(json: String) {
    println!("kafka: {}", json)
}
