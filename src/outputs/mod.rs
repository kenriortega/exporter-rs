use crate::config::Cfg;

pub mod kfk;

pub struct Postgres;

pub struct Console;

pub struct Output<T> {
    pub data_received: String,
    pub cfg: Cfg,
    state: std::marker::PhantomData<T>,
}

impl Output<Postgres> {
    pub fn send_data(&self) {
        println!("postgresql: {}", self.data_received)
    }
}

impl Output<Console> {
    pub fn send_data(&self) {
        println!("stdout: {}", self.data_received)
    }
}

impl<T> Output<T> {
    pub fn new(cfg: Cfg, data_received: String) -> Self {
        Output {
            data_received,
            cfg,
            state: Default::default(),
        }
    }
}
