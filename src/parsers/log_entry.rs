use chrono::DateTime;
use regex::Regex;
use serde::{Deserialize, Serialize};

pub trait LogTransformer<T> {
    fn parse_log_line(line: String) -> Option<T>;
    fn parse_to_json(entry: T) -> Option<String>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl LogTransformer<LogEntryNginx> for LogEntryNginx {
    fn parse_log_line(line: String) -> Option<LogEntryNginx> {
        let re =
            Regex::new(r#"^([\w:.]+) - (\S+) \[(.*?)] "(.*?)" (\d+) (\d+) "(.*?)" "(.*?)" (.*?)$"#)
                .ok()?;
        let captures = re.captures(line.as_str())?;
        let format = "%d/%b/%Y:%H:%M:%S %z";
        let date_hour_dt = DateTime::parse_from_str(&captures[3], format).unwrap();
        let timestamp = date_hour_dt.timestamp();
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntryApache {
    ip: String,
    identity: String,
    user: String,
    datetime: i64,
    method: String,
    path: String,
    protocol: String,
    status: u16,
    size: Option<u64>,
    referer: String,
    user_agent: String,
}

impl LogTransformer<LogEntryApache> for LogEntryApache {
    fn parse_log_line(line: String) -> Option<LogEntryApache> {
        let re =
            Regex::new(r#"^(\S+) (\S+) (\S+) \[([\w:/]+\s[+\-]\d{4})] "(\S+) (\S+)\s*(\S*)" (\d{3}) (\S+) "([^"]*)" "([^"]*)"#)
                .ok()?;
        let captures = re.captures(line.as_str())?;
        let format = "%d/%b/%Y:%H:%M:%S %z";
        let date_hour_dt = DateTime::parse_from_str(&captures[4], format).unwrap();
        let timestamp = date_hour_dt.timestamp();
        Some(LogEntryApache {
            ip: captures[1].to_string(),
            identity: captures[2].to_string(),
            user: captures[3].to_string(),
            datetime: timestamp,
            method: captures[5].to_string(),
            path: captures[6].to_string(),
            protocol: captures[7].to_string(),
            status: captures[8].parse().unwrap(),
            size: if captures[9] == "-".to_owned() {
                None
            } else {
                Some(captures[9].parse().unwrap())
            },
            referer: captures[10].to_string(),
            user_agent: captures[11].to_string(),
        })
    }
    fn parse_to_json(entry: LogEntryApache) -> Option<String> {
        serde_json::to_string(&entry).ok()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntryIIS {
    date: String,
    time: String,
    s_sitename: String,
    s_computername: String,
    s_ip: String,
    cs_method: String,
    cs_uri_stem: String,
    cs_uri_query: String,
    s_port: String,
    cs_username: String,
    c_ip: String,
    cs_version: String,
    cs_user_agent: String,
    cs_cookie: String,
    cs_referer: String,
    cs_host: String,
    sc_status: String,
    sc_substatus: String,
    sc_win32_status: String,
    sc_bytes: String,
    cs_bytes: String,
    time_taken: String,
}

impl LogTransformer<LogEntryIIS> for LogEntryIIS {
    fn parse_log_line(line: String) -> Option<LogEntryIIS> {
        let re =
            Regex::new(
                r#"^(\d{4}-\d{2}-\d{2})\s+(\d{2}:\d{2}:\d{2})\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+([\S\s]+?)\s+([\S\s]+?)\s+([\S\s]+?)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)$"#
            ).unwrap();
        if line.starts_with("#Software")
            || line.starts_with("#Version")
            || line.starts_with("#Date")
            || line.starts_with("#Fields")
        {
            None
        } else {
            let captures = re.captures(&line).unwrap();
            Some(LogEntryIIS {
                date: captures[0].to_string(),
                time: captures[1].to_string(),
                s_sitename: captures[2].to_string(),
                s_computername: captures[3].to_string(),
                s_ip: captures[4].to_string(),
                cs_method: captures[5].to_string(),
                cs_uri_stem: captures[6].to_string(),
                cs_uri_query: captures[7].to_string(),
                s_port: captures[8].to_string(),
                cs_username: captures[9].to_string(),
                c_ip: captures[10].to_string(),
                cs_version: captures[11].to_string(),
                cs_user_agent: captures[12].to_string(),
                cs_cookie: captures[13].to_string(),
                cs_referer: captures[14].to_string(),
                cs_host: captures[15].to_string(),
                sc_status: captures[16].to_string(),
                sc_substatus: captures[17].to_string(),
                sc_win32_status: captures[18].to_string(),
                sc_bytes: captures[19].to_string(),
                cs_bytes: captures[20].to_string(),
                time_taken: captures[21].to_string(),
            })
        }
    }
    fn parse_to_json(entry: LogEntryIIS) -> Option<String> {
        serde_json::to_string(&entry).ok()
    }
}
