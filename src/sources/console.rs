use crate::sources::datasource::Datasource;

pub struct DatasourceConsole;

impl Datasource for DatasourceConsole {
    fn send_data(&self, json: String) {
        println!("Stdout: {}", json)
    }
}
