use chrono::DateTime;
use chrono_tz::Tz;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LogEntryNginx {
    pub remote_addr: String,
    pub time_local: i64,
    pub request: String,
    pub status: u16,
    pub body_bytes_sent: u64,
    pub http_referer: String,
    pub http_user_agent: String,
    pub request_time: f32,
}

impl LogTransformer for LogEntryNginx {
    fn parse_log_line(line: String) -> Option<LogEntryNginx> {
        let re =
            Regex::new(r#"^([\w:.]+) - (\S+) \[(.*?)] "(.*?)" (\d+) (\d+) "(.*?)" "(.*?)" (.*?)$"#)
                .ok()?;
        let captures = re.captures(line.as_str())?;
        let format = "%d/%b/%Y:%H:%M:%S %z";
        let date_hour_dt = DateTime::parse_from_str(&captures[3], format).unwrap();
        let timezone = Tz::America__Havana;

        let timestamp = date_hour_dt.with_timezone(&timezone).timestamp();
        Some(LogEntryNginx {
            remote_addr: captures[1].to_owned(),
            time_local: timestamp,
            request: captures[4].to_owned(),
            status: captures[5].parse().unwrap(),
            body_bytes_sent: captures[6].parse().unwrap(),
            http_referer: captures[7].to_owned(),
            http_user_agent: captures[8].to_owned(),
            request_time: captures[9].parse().unwrap(),
        })
    }
    fn parse_to_json(entry: LogEntryNginx) -> Option<String> {
        serde_json::to_string(&entry).ok()
    }
}

pub trait LogTransformer {
    fn parse_log_line(line: String) -> Option<LogEntryNginx>;
    fn parse_to_json(entry: LogEntryNginx) -> Option<String>;
}
