use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fmt::format;
use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
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

fn read_file_log(paths: Vec<PathBuf>)-> io::Result<()> {

    for path in paths.iter() {
        println!("file to read {:?}", path.file_name());
        match path.file_name() {
            Some(name) => {
                if name == "access.metrics.log" {
                    let offset_file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(format!("_offset_{}.txt",name.to_str().unwrap()))?;
                    let mut offset_reader = BufReader::new(&offset_file);
                    let mut offset_str = String::new();
                    offset_reader.read_line(&mut offset_str)?;
                    let mut offset = offset_str.trim().parse::<usize>().unwrap_or(0);
                    let mut offset_file = BufWriter::new(&offset_file);

                    println!("last offset from txt : {}",offset);
                    let file = File::open(path).unwrap();
                    let reader = BufReader::new(file);
                    if offset > 0 {

                        for line in reader.lines().skip(offset - 1) {

                            match parse_log_line(line.unwrap().to_owned()) {
                                Some(entry) => {
                                    insert_into_db(parse_to_json(entry).expect("error"));
                                    offset += 1;
                                }
                                None => {}
                            };
                        }
                    } else {
                        for line in reader.lines() {

                            match parse_log_line(line.unwrap().to_owned()) {
                                Some(entry) => {
                                    insert_into_db(parse_to_json(entry).expect("error"));
                                    offset += 1;
                                }
                                None => {}
                            };
                        }
                    }


                    println!("last offset to commit {}",offset);
                    offset_file.flush()?;
                    offset_file.seek(SeekFrom::Start(0)).unwrap();
                    offset_file.write_fmt(format_args!("{}", offset)).unwrap();

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
