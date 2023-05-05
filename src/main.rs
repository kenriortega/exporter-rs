// mod notifier;
mod config;
mod parsers;
mod sources;

use crate::config::Cfg;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

fn main() {
    // Read config file
    let cfg: Cfg = Cfg::new();

    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    println!("watching {}", path);

    if let Err(e) = watch(path, cfg) {
        println!("error: {:?}", e)
    }
}

fn watch<P: AsRef<Path>>(path: P, cfg: Cfg) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => event_parser(event, cfg.clone()),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn event_parser(event: Event, cfg: Cfg) {
    let paths = event.paths;
    // READ files from paths changed
    match cfg.logs_type {
        config::LogType::Nginx => {
            parsers::nginx::read_file_log(paths, cfg).expect("error: read_file_log")
        }
        config::LogType::IIS => {
            println!("No {:?} parser implemented", config::LogType::IIS);
        }
        config::LogType::Apache => {
            println!("No {:?} parser implemented", config::LogType::Apache);
        }
        config::LogType::UnKnown => {
            println!("No {:?} parser implemented", config::LogType::UnKnown);
        }
    }
}
