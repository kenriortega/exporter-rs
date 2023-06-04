use crate::config::Cfg;
use crate::parsers::log_entry::{LogEntryApache, LogEntryIIS, LogEntryNginx};

pub mod console;
pub mod kfk;
pub mod pgx;
pub mod loki;

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

impl<T> Output<T> {
    pub async fn new(cfg: Cfg, data_received: LogType) -> Self {
        Output {
            data_received,
            cfg,
            state: Default::default(),
        }
    }
}
