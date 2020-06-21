use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    // Create a channel to receive the events.
    let (sender, receiver) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(sender, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch("/home/jpm/rust/rust-notes/src", RecursiveMode::Recursive)
        .unwrap();

    loop {
        match receiver.recv() {
            Ok(event) => {
                println!("{:?}", event);
                match event {
                    notify::DebouncedEvent::Write(p) => {
                        println!("Wrote to : {:?}", p);
                    }
                    _ => (),
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
