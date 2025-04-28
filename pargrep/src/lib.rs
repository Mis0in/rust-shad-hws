#![forbid(unsafe_code)]

use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
pub struct Match {
    pub path: PathBuf,
    pub line: String,
    pub line_number: usize,
}

#[derive(Debug)]
pub struct Error {
    pub path: PathBuf,
    pub error: io::Error,
}

pub enum Event {
    Match(Match),
    Error(Error),
}

fn rec_run(current_path: &PathBuf, search_pattern: &str, event_sender: Sender<Event>) {
    if current_path.is_file() {
        grep(current_path, search_pattern, event_sender.clone());
        return;
    }

    if let Ok(dir_entries) = current_path.read_dir() {
        dir_entries.par_bridge().for_each(|dir_entry| {
            if let Ok(dir_entry) = dir_entry {
                let entry_path = dir_entry.path();

                if entry_path.is_dir() {
                    rec_run(&entry_path, search_pattern, event_sender.clone());
                } else if entry_path.is_file() {
                    grep(&entry_path, search_pattern, event_sender.clone());
                }
            }
        });
    } else if let Err(read_err) = current_path.read_dir() {
        event_sender
            .send(Event::Error(Error {
                path: current_path.clone(),
                error: read_err,
            }))
            .unwrap();
    }
}

pub fn run<P: AsRef<Path>>(root_path: P, search_pattern: &str) -> Vec<Event> {
    let (event_sender, event_receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();
    let root_path = root_path.as_ref().to_path_buf();
    let search_pattern = search_pattern.to_string();

    let search_thread = thread::spawn(move || {
        rec_run(&root_path, &search_pattern, event_sender);
    });

    let collected_events: Vec<Event> = event_receiver.iter().collect();

    search_thread.join().unwrap();
    collected_events
}

fn grep(file_path: &PathBuf, search_pattern: &str, event_sender: Sender<Event>) {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            event_sender
                .send(Event::Error(Error {
                    path: file_path.clone(),
                    error: err,
                }))
                .unwrap();
            return;
        }
    };

    let buffer_reader = BufReader::new(file);
    buffer_reader
        .lines()
        .enumerate()
        .for_each(|(line_index, file_line)| {
            if let Ok(file_line) = file_line {
                if file_line.contains(search_pattern) {
                    let new_match = Match {
                        path: file_path.clone(),
                        line: file_line,
                        line_number: line_index + 1,
                    };
                    event_sender.send(Event::Match(new_match)).unwrap();
                }
            }
        });
}
