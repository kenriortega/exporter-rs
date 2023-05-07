use crate::config::Cfg;
use crate::parsers::log_entry::{LogEntryIIS, LogTransformer};
use crate::sources::datasource::{DatasourceFactory, SourceType};
use chrono::prelude::*;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::PathBuf;

pub fn read_file_log(paths: Vec<PathBuf>, cfg: Cfg) -> io::Result<()> {
    for path in paths.iter() {
        println!("file to read {:?}", path.file_name());
        match path.file_name() {
            Some(name) => {
                let local: DateTime<Local> = Local::now();
                let formatted_date = local.format("%Y-%m-%d").to_string();
                let offset_file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(format!(
                        "_offset_{}_{}.txt",
                        name.to_str().unwrap(),
                        formatted_date
                    ))?;
                let mut offset_reader = BufReader::new(&offset_file);
                let mut offset_str = String::new();
                offset_reader.read_line(&mut offset_str)?;
                let mut offset = offset_str.trim().parse::<usize>().unwrap_or(0);
                let mut offset_file = BufWriter::new(&offset_file);

                println!("last offset from txt : {}", offset);
                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);
                if offset > 0 {
                    for line in reader.lines().skip(offset - 1) {
                        match LogEntryIIS::parse_log_line(line?.to_owned()) {
                            Some(entry) => {
                                send_to_datasource(entry, cfg.clone());
                                offset += 1;
                            }
                            None => {
                                println!("This line can`t be parsed")
                            }
                        };
                    }
                } else {
                    for line in reader.lines() {
                        match LogEntryIIS::parse_log_line(line?.to_owned()) {
                            Some(entry) => {
                                send_to_datasource(entry, cfg.clone());
                                offset += 1;
                            }
                            None => {
                                println!("This line can`t be parsed")
                            }
                        };
                    }
                }

                println!("last offset to commit {}", offset);
                offset_file.flush()?;
                offset_file.seek(SeekFrom::Start(0)).unwrap();
                offset_file.write_fmt(format_args!("{}", offset))?;
            }
            None => {}
        }
    }
    Ok(())
}

fn send_to_datasource(entry: LogEntryIIS, cfg: Cfg) {
    let mut sc: Vec<SourceType> = Vec::new();
    for data_source in cfg.data_sources {
        sc.push(SourceType::from_string(&data_source));
    }

    let json_data = LogEntryIIS::parse_to_json(entry.clone()).expect("error");

    // using Factory pattern
    for s in sc.iter() {
        let datasource = DatasourceFactory::create_ds(&s);
        // Datasource notifier
        datasource.send_data(json_data.clone())
    }
}
