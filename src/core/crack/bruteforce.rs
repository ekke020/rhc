use std::{
    char::from_u32,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Instant,
};

use crate::{
    algorithm::{self, Algorithm},
    core::{
        constants::{ASCII_95_TABLE, NO_SPECIAL_RANGE},
        package::Package,
    },
};

use super::result::PasswordMatch;

pub struct BruteForce<'a> {
    target: &'a Vec<u8>,
    range: &'static [u8],
    counter: Arc<AtomicU32>,
    instant: Instant,
    algorithm: Box<dyn Algorithm>,
}

impl<'a> BruteForce<'a> {
    pub fn from(
        target: &'a Vec<u8>,
        range: &'static [u8],
        counter: Arc<AtomicU32>,
        algorithm: Box<dyn Algorithm>,
    ) -> Self {
        Self {
            target,
            range,
            counter,
            instant: Instant::now(),
            algorithm,
        }
    }
    pub fn run(&mut self) -> Option<PasswordMatch> {
        let mut n = 4;
        let pm = 'runner: loop {
            for c in self.range {
                let mut word = Vec::with_capacity(n);
                word.push(*c);
                if let Some(result) = self.calculate(None, n - 1, &mut word) {
                    break 'runner result;
                }
            }
            n += 1;
        };
        Some(pm)
    }

    fn calculate(
        &mut self,
        pm: Option<PasswordMatch>,
        length: usize,
        word: &mut Vec<u8>,
    ) -> Option<PasswordMatch> {
        if pm.is_some() {
            return pm;
        } else if length == 0 {
            let algorithm = self.algorithm.as_mut();
            algorithm.populate(word.as_slice());
            algorithm.execute();
            algorithm.compare(self.target);
            PasswordMatch::from(
                password.to_string(),
                self.algorithm.to_string(),
                self.target.to_vec(),
            );
            return None;
        }
        NO_SPECIAL_RANGE.iter().for_each(|c| {
            word.push(*c);
            self.calculate(None, length - 1, word);
            word.pop();
        });
        None
    }

    fn execute_comparison(&self, word: Vec<u8>) -> Option<PasswordMatch> {
        let algorithm = self.algorithm.as_mut();
        algorithm.populate(word.as_slice());
        algorithm.execute();
        match algorithm.compare(self.target) {
            true => self.create_password_match(),
            false => None,
        }
    }

    fn create_password_match(&self) -> PasswordMatch {
        PasswordMatch::from(
            password.to_string(),
            self.algorithm.to_string(),
            self.target.to_vec(),
        )
    }
}

fn calculate(
    bf: &mut BruteForce,
    pm: Option<PasswordMatch>,
    length: usize,
    word: &mut Vec<u8>,
) -> Option<PasswordMatch> {
    if pm.is_some() {
        return pm;
    } else if length == 0 {
        return None;
    }
    NO_SPECIAL_RANGE.iter().for_each(|c| {
        word.push(*c);
        calculate(bf, None, length - 1, word);
        word.pop();
    });
    None
}


fn execute_comparison(bf: &BruteForce, word: Vec<u8>) -> Option<PasswordMatch> {
    let algorithm = bf.algorithm.as_mut();
    algorithm.populate(word.as_slice());
    algorithm.execute();
    match algorithm.compare(bf.target) {
        true => todo!(),
        false => None,
    }
}