mod constants;
mod error;
use self::error::core::CoreError;
use crate::cli::settings::GlobalSettings;
use constants::ASCII_95_TABLE;
use std::{char::from_u32, str::from_utf8, thread};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub fn run(settings: GlobalSettings) {
    if let Err(e) = core(settings) {
        println!("{}", e);
        std::process::exit(e.get_exit_code());
    }
}

fn core(settings: GlobalSettings) -> Result<(), CoreError> {
    let atomic_value = Arc::new(AtomicU32::new(0));
    let num_cores = num_cpus::get();
    let chunk_size = ASCII_95_TABLE.len() / num_cores;
    let mut threads = vec![];

    for i in 0..=num_cores {
        let start = chunk_size * i;
        let end = if i == num_cores {
            ASCII_95_TABLE.len() - 1
        } else {
            chunk_size * (i + 1)
        };
        let chunk = &ASCII_95_TABLE[start..end];
        let atomic_value = Arc::clone(&atomic_value);
        threads.push(thread::spawn(move || {
            for c in chunk {
                let char = from_u32(*c).unwrap();
                run_crack(5, char.to_string(), &mut false, &atomic_value);
            }
        }));
    }
    println!("I have spawned all threads!");
    for thread in threads {
        thread.join().unwrap();
    }

    Ok(())
}

pub fn run_crack(length: usize, mut word: String, is_cracked: &mut bool, count: &Arc<AtomicU32>) {
    if *is_cracked {
        return;
    } else if length == 0 {
        let total = count.fetch_add(1, Ordering::SeqCst);
        test_print(&word, total);
        return;
    }

    ASCII_95_TABLE.iter().for_each(|c| {
        word.push(from_u32(*c).unwrap());
        run_crack(length - 1, word.clone(), is_cracked, count);
        word.pop();
    });
}

fn test_print(word: &str, count: u32) {
    let mut sha = crate::sha2::Sha224::new(word);
    sha.run();
    if count % 1000000 == 0 {
        println!("I have hashed: {} words", count);
    }
}
