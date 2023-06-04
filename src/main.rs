
use exporter_core::config::sources::LogType;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use exporter_core::config::Cfg;
use exporter_core::parsers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read config file
    let cfg = Cfg::new().await;

    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    println!("watching {}", path);

    if let Err(e) = watch(path, cfg).await {
        println!("error: {:?}", e)
    }

    Ok(())
}

async fn watch<P: AsRef<Path>>(path: P, cfg: Cfg) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event_file) => event_file_log_to_parse(event_file, cfg.clone()).await,
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

async fn event_file_log_to_parse(event: Event, cfg: Cfg) {
    let paths = event.paths;

    match cfg.logs_type {
        LogType::Nginx => {
            parsers::nginx::read_file_log(paths, cfg).await.unwrap();
        }
        LogType::IIS => {
            parsers::iis::read_file_log(paths, cfg).await.unwrap();
        }
        LogType::Apache => {
            parsers::apache2::read_file_log(paths, cfg).await.unwrap();
        }
        LogType::UnKnown => {
            println!("No {:?} parser implemented", LogType::UnKnown);
        }
    }
}
