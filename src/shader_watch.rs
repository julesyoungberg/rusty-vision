use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

#[path = "util.rs"]
mod util;

#[path = "shaders.rs"]
mod shaders;

pub fn watch() {
    thread::spawn(|| {
        let (tx, rx) = channel();

        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

        watcher
            .watch(shaders::SHADERS_PATH, RecursiveMode::Recursive)
            .unwrap();

        loop {
            match rx.recv() {
                Ok(event) => println!("{:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}
