use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct LogEntry {
    remote_addr: String,
    time_local: String,
    request: String,
    status: u16,
    body_bytes_sent: u64,
    http_referer: String,
    http_user_agent: String,
    request_time: f32,
}
fn main() {
    match offset_committer_check() {
        Ok(offset) => println!("{}", offset),
        Err(e) => println!("watch error: {:?}", e),
    }
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    println!("watching {}", path);
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => event_parser(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn event_parser(event: Event) {
    let paths = event.paths;
    // READ files from paths changed
    read_file_log(paths).expect("TODO: panic message");
}

fn read_file_log(paths: Vec<PathBuf>) -> Result<()> {
    for path in paths.iter() {
        println!("file to read {:?}", path.file_name());
        match path.file_name() {
            Some(name) => {
                if name == "access.metrics.log" {
                    let file = File::open(path).unwrap();
                    let reader = BufReader::new(file);
                    for (i, line) in reader.lines().enumerate() {
                        // parse logfile to an struct

                        match parse_log_line(line.unwrap().to_owned()) {
                            Some(entry) => {
                                insert_into_db(parse_to_json(entry).expect("error"));
                                // commit offset
                                offset_committer_file(i).expect("TODO: panic message");
                            }
                            None => {}
                        };
                    }
                } else {
                    println!(
                        "This log file {}, don`t have an implemented parser for this moment",
                        name.to_str().unwrap()
                    );
                }
            }
            None => {}
        }
    }
    Ok(())
}

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

fn insert_into_db(json: String) {
    println!("{}", json)
}

fn offset_committer_file(line_number: usize) -> Result<()> {
    let ln = format!("{}", line_number);
    let mut file = File::create("_offset.txt").unwrap();

    file.write(ln.as_ref()).unwrap();
    Ok(())
}

fn offset_committer_check() -> Result<String> {
    if let Ok(file) = File::open("_offset.txt") {
        if let Some(Ok(line)) = BufReader::new(file).lines().next() {
            return Ok(line);
        }
    } else {
        File::create("_offset.txt").unwrap();
        return Ok("".to_string());
    }
    Ok("".to_string())
}
