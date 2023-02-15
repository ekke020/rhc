use std::{
    char::from_u32,
    str::from_utf8,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Instant,
};

use crate::{
    algorithm::{self, Algorithm},
    core::package::Package,
};

use super::{consts::Table, result::PasswordMatch, wrapper::Crack};

pub struct Incremental {
    target: Vec<u8>,
    range: &'static [u8],
    table: Table,
    counter: Option<Arc<AtomicU32>>,
    instant: Instant,
    algorithm: Box<dyn Algorithm>,
    start_length: Option<usize>,
}

impl Crack for Incremental {
    fn from(package: Package, index: usize) -> Self {
        let (start, end) = get_ascii_span(index);
        Self {
            target: package.get_target().to_vec(),
            range: &super::consts::NO_SPECIAL_TABLE[start..end],
            table: super::consts::NO_SPECIAL_TABLE, // TODO: This will be hardcoded for now...
            counter: None,
            instant: Instant::now(),
            algorithm: package.get_algorithm(),
            start_length: None,
        }
    }

    fn run(&mut self) -> Option<PasswordMatch> {
        let mut n = self.start_length.unwrap_or(1);
        let pm = 'runner: loop {
            for c in self.range {
                let mut word = Vec::with_capacity(n);
                word.push(*c);
                if let Some(result) = self.calculate(None, n - 1, &mut word) {
                    break 'runner result;
                }
            }
            println!("I just increased the wordsize to {}", n);
            n += 1;
        };
        Some(pm)
    }
}

impl Incremental {
    fn calculate(
        &mut self,
        pm: Option<PasswordMatch>,
        length: usize,
        word: &mut Vec<u8>,
    ) -> Option<PasswordMatch> {
        if pm.is_some() {
            return pm;
        } else if length == 0 {
            return self.execute_comparison(word);
        }

        self.table.iter().find_map(|b| {
            word.push(*b);
            let result = self.calculate(None, length - 1, word);
            word.pop();
            result
        })
    }

    fn execute_comparison(&mut self, word: &Vec<u8>) -> Option<PasswordMatch> {
        let algorithm = self.algorithm.as_mut();
        algorithm.populate(word.as_slice());
        algorithm.execute();
        match algorithm.compare(&self.target) {
            true => Some(self.create_password_match(word)),
            false => None,
        }
    }

    fn create_password_match(&self, word: &Vec<u8>) -> PasswordMatch {
        // TODO: This unwrap is a bit risky, consider moving the value as a vector instead.
        let password = from_utf8(word.as_slice()).unwrap();
        PasswordMatch::from(
            password,
            self.algorithm.to_string(),
            &self.target,
            self.instant.elapsed().as_secs(),
        )
    }
}

// TODO: This is a temporary hack...
fn get_ascii_span(index: usize) -> (usize, usize) {
    let num_cores = num_cpus::get();
    let chunk_size = super::consts::NO_SPECIAL_TABLE.len() / num_cores;

    let start = chunk_size * index;
    let end = if index == num_cores {
        super::consts::NO_SPECIAL_TABLE.len()
    } else {
        chunk_size * (index + 1)
    };
    (start, end)
}
