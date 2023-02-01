use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::{char::from_u32, str::from_utf8, thread};

use super::constants::ASCII_95_TABLE;
use super::error::core::CoreError;
use super::package::Package;

fn brute_force_job(package: Package) -> Result<Vec<JoinHandle<()>>, CoreError> {
    let atomic_value = Arc::new(AtomicU32::new(0));
    let num_cores = num_cpus::get();
    let mut threads = vec![];

    for i in 0..=num_cores {
        let counter = Arc::clone(&atomic_value);
        let p = package.clone();
        threads.push(thread::spawn(move || {
            // Create an instance of bruteforce and run it on the thread.
            // Crack::from(get_ascii_span(i), counter, p).run();
        }));
    }
    Ok(threads)
}

fn get_ascii_span(index: usize) -> (usize, usize) {
    let num_cores = num_cpus::get();
    let chunk_size = ASCII_95_TABLE.len() / num_cores;

    let start = chunk_size * index;
    let end = if index == num_cores {
        ASCII_95_TABLE.len() - 1
    } else {
        chunk_size * (index + 1)
    };
    (start, end)
}

fn resource_job(package: Package) -> Result<Vec<JoinHandle<()>>, CoreError> {
    let num_cores = num_cpus::get();
    let mut threads = vec![];

    for i in 0..=num_cores {
        let p = package.clone();
        threads.push(thread::spawn(move || {
            // Create an instance of resource and run it on the thread.
            // Crack::from(get_ascii_span(i), counter, p).run();
        }));
    }
    Ok(threads)
}