use std::str::from_utf8;

use crate::algorithm::{self, Algorithm};

use super::result::PasswordMatch;

pub struct Dictionary<'a> {
    target: &'a Vec<u8>,
    wordlist: &'a [String],
    algorithm: Box<dyn Algorithm>,
}

impl<'a> Dictionary<'a> {}

impl<'a> Dictionary<'a> {
    pub fn from(target: &'a Vec<u8>, wordlist: &'a [String], algorithm: Box<dyn Algorithm>) -> Self {
        Self {
            target,
            wordlist,
            algorithm,
        }
    }

    pub fn run(&mut self) -> Option<PasswordMatch> {
        self.wordlist
            .iter()
            .find_map(|word| self.execute_comparison(&word.as_bytes()))
    }

    fn execute_comparison(&mut self, word: &[u8]) -> Option<PasswordMatch> {
        let algorithm = self.algorithm.as_mut();
        algorithm.populate(word);
        algorithm.execute();
        match algorithm.compare(self.target) {
            true => Some(self.create_password_match(word)),
            false => None,
        }
    }

    fn create_password_match(&self, word: &[u8]) -> PasswordMatch {
        // TODO: This unwrap is a bit risky, consider moving the value as a vector instead.
        let password = from_utf8(word).unwrap();
        PasswordMatch::from(password, self.algorithm.to_string(), self.target)
    }
}
