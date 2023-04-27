use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use regex::Regex;

#[derive(Debug)]
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
    read_file_log(paths);
}

fn read_file_log(paths: Vec<PathBuf>) {
    // TODO: learn how to read all content in the file first time and then only read by last line (like and offset)
    for path in paths.iter() {
        println!("file to read {:?}", path.file_name());
        match path.file_name() {
            Some(name) => {
                if name == "access.metrics.log" {
                    let file = File::open(path).unwrap();
                    let reader = BufReader::new(file);
                    for (i, line) in reader.lines().enumerate() {
                        // parse logfile to an struct
                       match parse_log_line(i, line.unwrap().to_owned()) {
                           Some(entry) => println!("Line: {}, Entry: {:?}",i,entry),
                           None=> {}
                       } ;
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
}

fn parse_log_line(i: usize, line: String) -> Option<LogEntry>{
    println!("number line: {}, content_log: {}", i, line);
    let re = Regex::new(r#"^([\w:.]+) - (\S+) \[(.*?)\] "(.*?)" (\d+) (\d+) "(.*?)" "(.*?)" (.*?)$"#).unwrap();
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
