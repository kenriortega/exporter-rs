use crate::outputs::{LogType, Output};

pub struct Loki;

impl Output<Loki> {
    pub async fn send_data(&self) {
        match &self.data_received {
            LogType::LogEntryApache(data) => {
                println!("loki: {:?}", data)
            }
            LogType::LogEntryIIS(data) => {
                println!("loki: {:?}", data)
            }
            LogType::LogEntryNginx(data) => {
                println!("loki: {:?}", data)
            }
        }
    }
}
