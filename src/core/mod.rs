mod constants;
mod crack;
mod error;
pub mod setup;
use self::crack::Crack;
use self::error::core::CoreError;
use crate::cli::settings::GlobalSettings;
use constants::ASCII_95_TABLE;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::{char::from_u32, str::from_utf8, thread};

pub fn run(mut settings: GlobalSettings) {
    if let Err(e) = core(settings) {
        println!("{}", e);
        std::process::exit(e.get_exit_code());
    }
}

fn core(settings: GlobalSettings) -> Result<(), CoreError> {
    let atomic_value = Arc::new(AtomicU32::new(0));
    let num_cores = num_cpus::get();
    let mut threads = vec![];
    let package = setup::Package::assemble(settings)?;

    for i in 0..=num_cores {
        let counter = Arc::clone(&atomic_value);
        let p = package.clone();
        threads.push(thread::spawn(move || {
            Crack::from(get_ascii_span(i), counter, p).run();
        }));
    }
    for thread in threads {
        let result = thread.join().unwrap();
    }

    Ok(())
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
