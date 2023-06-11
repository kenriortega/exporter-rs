pub enum SourceType {
    Kafka,
    Postgresql,
    Stdout,
    Loki,
}

impl SourceType {
    pub fn from_string(source: &str) -> Self {
        let value = match source {
            "Kafka" => SourceType::Kafka,
            "Postgresql" => SourceType::Postgresql,
            "Loki" => SourceType::Loki,
            _ => SourceType::Stdout,
        };
        value
    }
}

#[derive(Debug, Clone)]
pub enum LogType {
    Nginx,
    IIS,
    Apache,
    UnKnown,
}

impl LogType {
    pub fn from_string(sink: &str) -> Self {
        let value = match sink {
            "nginx" => LogType::Nginx,
            "iis" => LogType::IIS,
            "apache" => LogType::Apache,
            _ => LogType::UnKnown,
        };
        value
    }
}
