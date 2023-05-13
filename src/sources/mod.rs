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
