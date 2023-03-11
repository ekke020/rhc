use std::str::from_utf8;

use crate::algorithm::Algorithm;

use super::{charset::Table, result::PasswordMatch, setup::IncrementalSettings};

pub struct Incremental<'a> {
    target: &'a Vec<u8>,
    range: &'static [u8],
    table: Table,
    algorithm: Box<dyn Algorithm>,
    min_length: usize,
    max_length: usize,
}

impl <'a>Incremental<'a> {
    pub fn from(target: &'a Vec<u8>, settings: &IncrementalSettings, algorithm: Box<dyn Algorithm>) -> Self {
        Self {
            target,
            range: settings.range(),
            table: settings.table(),
            algorithm,
            min_length: settings.min_length(),
            max_length: settings.max_length(),
        }
    }

    pub fn run(&mut self) -> Option<PasswordMatch> {
        let mut n = self.min_length;
        'runner: loop {
            for c in self.range {
                let mut word = Vec::with_capacity(n);
                word.push(*c);
                if let Some(result) = self.calculate(None, n - 1, &mut word) {
                    break 'runner Some(result);
                }
            }
            n += 1;
            if (n > self.max_length) {
                break 'runner None;
            }
        }
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
            return self.execute_comparison(word);
        }

        self.table.iter().find_map(|b| {
            word.push(*b);
            let result = self.calculate(None, length - 1, word);
            word.pop();
            result
        })
    }

    fn execute_comparison(&mut self, word: &[u8]) -> Option<PasswordMatch> {
        let algorithm = self.algorithm.as_mut();
        algorithm.populate(word);
        algorithm.execute();
        match algorithm.compare(&self.target) {
            true => Some(self.create_password_match(word)),
            false => None,
        }
    }

    fn create_password_match(&self, word: &[u8]) -> PasswordMatch {
        // TODO: This unwrap is a bit risky, consider moving the value as a vector instead.
        let password = from_utf8(word).unwrap();
        PasswordMatch::from(
            password,
            self.algorithm.to_string(),
            &self.target,
        )
    }
}
