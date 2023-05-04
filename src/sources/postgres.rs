use crate::sources::datasource::Datasource;

pub struct DatasourcePostgres;

impl Datasource for DatasourcePostgres {
    fn send_data(&self, json: String) {
        println!("postgres: {}", json)
    }
}

pub fn send_data(json: String) {
    println!("postgres: {}", json)
}
