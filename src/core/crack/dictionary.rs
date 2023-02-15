use std::str::from_utf8;

use crate::{
    algorithm::{self, Algorithm},
    core::{error::core::CoreError, package::Package},
};

use super::{consts::Table, result::PasswordMatch, wrapper::Crack};

pub struct Dictionary<'a> {
    target: &'a Vec<u8>,
    wordlist: &'a Vec<String>,
    algorithm: Box<dyn Algorithm>,
}

impl<'a> Crack for Dictionary<'a> {
    fn from(package: Package, index: usize) -> Self {
        todo!()
    }

    fn run(&mut self) -> Option<PasswordMatch> {
        self.wordlist
            .iter()
            .find_map(|word| self.execute_comparison(&word.as_bytes().to_vec()))
    }
}

impl<'a> Dictionary<'a> {
    fn execute_comparison(&mut self, word: &Vec<u8>) -> Option<PasswordMatch> {
        let algorithm = self.algorithm.as_mut();
        algorithm.populate(word.as_slice());
        algorithm.execute();
        match algorithm.compare(self.target) {
            true => Some(self.create_password_match(word)),
            false => None,
        }
    }

    fn create_password_match(&self, word: &Vec<u8>) -> PasswordMatch {
        // TODO: This unwrap is a bit risky, consider moving the value as a vector instead.
        let password = from_utf8(word.as_slice()).unwrap();
        PasswordMatch::from(password, self.algorithm.to_string(), self.target, 0)
    }
}
