use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{char::from_u32, str::from_utf8, thread};
use std::sync::mpsc::{self, Receiver};

use crate::algorithm::{self, Algorithm};

use super::constants::ASCII_95_TABLE;
use super::crack;
use super::crack::incremental::Incremental;
use super::crack::dictionary::Dictionary;
use super::crack::result::PasswordMatch;
use super::error::core::CoreError;
use super::package::Package;

pub(super) fn brute_force_job(
    package: Package,
) -> Receiver<Option<PasswordMatch>> {
    let atomic_value = Arc::new(AtomicU32::new(0));
    let atomic_instant = Arc::new(std::time::Instant::now());
    let (tx, rx) = mpsc::channel();
    let num_cores = num_cpus::get();
    let mut threads = vec![];
    for i in 0..num_cores {
        let counter = Arc::clone(&atomic_value);
        let instant = Arc::clone(&atomic_instant);
        let p = package.clone();
        let tx = tx.clone();
        let (start, end) = get_ascii_span(i);
        threads.push(thread::spawn(move || {
            // Create an instance of bruteforce and run it on the thread.
            let algorithm = p.get_algorithms().get(0).unwrap().get_algorithm();
            let target = p.get_target();
            println!("Thread {}: {:?}",i ,&ASCII_95_TABLE[start..end]);
            let mut bruteforce =
                Incremental::from(target, &ASCII_95_TABLE[start..end], counter, algorithm);
            let result = bruteforce.run();
            tx.send(result);
        }));
    };
    rx
}

fn get_ascii_span(index: usize) -> (usize, usize) {
    let num_cores = num_cpus::get();
    let chunk_size = ASCII_95_TABLE.len() / num_cores;

    let start = chunk_size * index;
    let end = if index == num_cores {
        ASCII_95_TABLE.len()
    } else {
        chunk_size * (index + 1)
    };
    (start, end)
}

pub(super) fn resource_job(
    package: Package,
    wordlist: Vec<String>,
) -> Result<Vec<JoinHandle<Option<PasswordMatch>>>, CoreError> {
    let num_cores = num_cpus::get();
    let chunk_size = wordlist.len() / num_cores;
    let mut threads = vec![];
    // TODO: This needs to be optimized. Need a way to split the wordlist more evenly.
    let mut chunks = wordlist
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<Vec<String>>>();

    let mut wordlist_rf = Arc::new(Mutex::new(chunks));

    for i in 0..=num_cores {
        let p = package.clone();
        let wordlist = wordlist_rf.clone();
        threads.push(thread::spawn(move || {
            // Create an instance of resource and run it on the thread.
            let target = p.get_target();
            let chunk = wordlist.lock().unwrap().pop().unwrap();
            let mut resources: Vec<Dictionary> = p
                .get_algorithms()
                .iter()
                .map(|algorithm| algorithm.get_algorithm())
                .map(|algorithm| Dictionary::from(target, &chunk, algorithm))
                .collect();
            for mut resource in resources {
                if let Some(result) = resource.run() {
                    return Some(result);
                }
            }
            None
        }));
    }
    Ok(threads)
}
