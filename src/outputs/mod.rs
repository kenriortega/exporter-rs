use crate::config::Cfg;
use crate::parsers::log_entry::{LogEntryApache, LogEntryIIS, LogEntryNginx};

pub mod kfk;
pub mod pgx;

pub struct Postgres;

pub struct Console;

pub enum LogType {
    LogEntryApache(LogEntryApache),
    LogEntryIIS(LogEntryIIS),
    LogEntryNginx(LogEntryNginx),
}

pub struct Output<T> {
    pub data_received: LogType,
    pub cfg: Cfg,
    state: std::marker::PhantomData<T>,
}

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

impl<T> Output<T> {
    pub async fn new(cfg: Cfg, data_received: LogType) -> Self {
        Output {
            data_received,
            cfg,
            state: Default::default(),
        }
    }
}
