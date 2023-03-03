use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

// track fs event in directories under pwd recursively.
fn main() -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(".".as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => println!("event tracked: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}
