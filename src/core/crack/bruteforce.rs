use std::{
    char::from_u32,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use crate::core::{package::Package, constants::ASCII_95_TABLE};

pub(super) struct BruteForce {
    start: usize,
    end: usize,
    counter: Arc<AtomicU32>,
    package: Package,
}

impl BruteForce {
    pub fn from(ascii_span: (usize, usize), counter: Arc<AtomicU32>, package: Package) -> Self {
        Self { 
            start: ascii_span.0,
            end: ascii_span.1,
            counter,
            package,
         }
    }

    pub fn run(&self) {
        let chunk = &ASCII_95_TABLE[self.start..self.end];
        for c in chunk {
            let char = from_u32(*c).unwrap();
            run_crack(5, char.to_string(), &mut false, &self.counter);
        }
    }
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
    let mut sha = crate::sha2::Sha224::from(word);
    sha.run();
    let t = sha.extract();
    if count % 1000000 == 0 {
        println!("I have hashed: {} words", count);
    }
}
