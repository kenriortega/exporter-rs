use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub remote_addr: String,
    pub time_local: String,
    pub request: String,
    pub status: u16,
    pub body_bytes_sent: u64,
    pub http_referer: String,
    pub http_user_agent: String,
    pub request_time: f32,
}

impl LogTransformer for LogEntry {
    fn parse_log_line(line: String) -> Option<LogEntry> {
        let re =
            Regex::new(r#"^([\w:.]+) - (\S+) \[(.*?)] "(.*?)" (\d+) (\d+) "(.*?)" "(.*?)" (.*?)$"#)
                .ok()?;
        let captures = re.captures(line.as_str())?;

        Some(LogEntry {
            remote_addr: captures[1].to_owned(),
            time_local: captures[3].to_owned(),
            request: captures[4].to_owned(),
            status: captures[5].parse().unwrap(),
            body_bytes_sent: captures[6].parse().unwrap(),
            http_referer: captures[7].to_owned(),
            http_user_agent: captures[8].to_owned(),
            request_time: captures[9].parse().unwrap(),
        })
    }
    fn parse_to_json(entry: LogEntry) -> Option<String> {
        serde_json::to_string(&entry).ok()
    }
}

pub trait LogTransformer {
    fn parse_log_line(line: String) -> Option<LogEntry>;
    fn parse_to_json(entry: LogEntry) -> Option<String>;
}
