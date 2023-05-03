use crate::parsers::log_entry::{LogEntry, LogTransformer};
use crate::sources::postgres;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::PathBuf;
pub fn read_file_log(paths: Vec<PathBuf>) -> io::Result<()> {
    for path in paths.iter() {
        println!("file to read {:?}", path.file_name());
        match path.file_name() {
            Some(name) => {
                if name == "access.metrics.log" {
                    let offset_file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(format!("_offset_{}.txt", name.to_str().unwrap()))?;
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
                            match LogEntry::parse_log_line(line.unwrap().to_owned()) {
                                Some(entry) => {
                                    postgres::insert_into_db(
                                        LogEntry::parse_to_json(entry).expect("error"),
                                    );
                                    offset += 1;
                                }
                                None => {}
                            };
                        }
                    } else {
                        for line in reader.lines() {
                            match LogEntry::parse_log_line(line.unwrap().to_owned()) {
                                Some(entry) => {
                                    postgres::insert_into_db(
                                        LogEntry::parse_to_json(entry).expect("error"),
                                    );
                                    offset += 1;
                                }
                                None => {}
                            };
                        }
                    }

                    println!("last offset to commit {}", offset);
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
