use crate::outputs::{LogType, Output};

pub struct Console;

impl Output<Console> {
    pub async fn send_data(&self) {
        match &self.data_received {
            LogType::LogEntryApache(data) => {
                println!("stdout: {:?}", data)
            }
            LogType::LogEntryIIS(data) => {
                println!("stdout: {:?}", data)
            }
            LogType::LogEntryNginx(data) => {
                println!("stdout: {:?}", data)
            }
        }
    }
}
