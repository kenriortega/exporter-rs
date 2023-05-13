pub struct Kafka;
pub struct Postgres;
pub struct Console;

pub struct Output<T> {
    pub data_received: String,
    state: std::marker::PhantomData<T>,
}

impl Output<Kafka> {
    pub fn send_data(&self) {
        println!("kafka: {}", self.data_received)
    }
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
    pub fn new(data_received: String) -> Self {
        Output {
            data_received,
            state: Default::default(),
        }
    }
}
