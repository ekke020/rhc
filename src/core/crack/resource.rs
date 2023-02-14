use crate::{
    algorithm::{self, Algorithm},
    core::{error::core::CoreError, package::Package},
};

use super::result::PasswordMatch;

pub struct Resource<'a> {
    target: &'a Vec<u8>,
    wordlist: &'a Vec<String>,
    algorithm: Box<dyn Algorithm>,
}

impl<'a> Resource<'a> {
    pub fn from(
        target: &'a Vec<u8>,
        wordlist: &'a Vec<String>,
        algorithm: Box<dyn Algorithm>,
    ) -> Self {
        Self {
            target,
            wordlist,
            algorithm,
        }
    }

    pub fn run(&mut self) -> Option<PasswordMatch> {
        let word = self
            .wordlist
            .iter()
            .find(|word| algorithm::execute_comparison(self.algorithm.as_mut(), word.as_bytes(), self.target));
        match word {
            Some(password) => Some(PasswordMatch::from(
                password.to_string(),
                self.algorithm.to_string(),
                self.target.to_vec(),
            )),
            None => None,
        }
    }
}
