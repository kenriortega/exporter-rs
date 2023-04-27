use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, Event};
use std::path::Path;

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

fn event_parser(event: Event){
    println!("changed: {:?}", event)
    // parse the events related with log.file (should be Modify)?
    // get paths and foreach all logs files and read content
    // parse all lines (using patterns or regex) (format apache, nginx, IIS) also JSON?
    // Use a database (like sqlite) to store last line check before close file (create struct)
    // Send data to different sources (kafka, loki, postgresql, should be elastic)
    // Create configuration file
    // Create dockerfile for this app and convert to daemon set for k8s.
}