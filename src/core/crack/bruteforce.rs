use std::{
    char::from_u32,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use crate::{
    algorithm::{self, Algorithm},
    core::{constants::ASCII_95_TABLE, package::Package},
};

use super::result::PasswordMatch;

pub struct BruteForce<'a> {
    target: &'a Vec<u8>,
    range: &'static [u32],
    counter: Arc<AtomicU32>,
    instant: Arc<std::time::Instant>,
    algorithm: Box<dyn Algorithm>,
}

impl<'a> BruteForce<'a> {
    pub fn from(
        target: &'a Vec<u8>,
        range: &'static [u32],
        counter: Arc<AtomicU32>,
        instant: Arc<std::time::Instant>,
        algorithm: Box<dyn Algorithm>,
    ) -> Self {
        Self {
            target,
            range,
            counter,
            instant,
            algorithm,
        }
    }
    pub fn run(&mut self) -> Option<PasswordMatch> {
        for c in self.range {
            let char = from_u32(*c).unwrap();
            crack(self, None, 4,char.to_string());
        }
        None
    }
}

fn crack(
    bf: &mut BruteForce,
    cracked: Option<String>,
    length: usize,
    mut word: String,
) -> Option<String> {
    if cracked.is_some() {
        return cracked;
    } else if length == 0 {
        let total = bf.counter.fetch_add(1, Ordering::SeqCst);
        let test = algorithm::execute_comparison(bf.algorithm.as_mut(), &word, bf.target);
        if test {
            println!("got em: {}", word);
            elapsed_time(bf.instant.elapsed().as_secs());
            std::process::exit(0);
        }
        if total % 10000000 == 0 {
            println!("I have hashed: {} words", total);
            println!("Current word: {}", word);
            println!("Current length: {}", word.len());
        }
        return None;
    }

    ASCII_95_TABLE.iter().for_each(|c| {
        word.push(from_u32(*c).unwrap());
        crack(bf, None, length - 1, word.clone());
        word.pop();
    });
    None
}


fn elapsed_time(elapsed: u64) {
    let seconds = (elapsed % 3600) % 60;
    let minutes = (elapsed % 3600 - seconds) / 60;
    let hours = (elapsed - minutes * 60 + seconds) / 3600;
    println!("H: {}, M: {}, S: {}", hours, minutes, seconds);
}