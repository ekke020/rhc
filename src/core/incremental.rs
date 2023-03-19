use crate::central::setup::ThreadSettings;
use crate::{
    algorithm::Algorithm,
    central::{IncrementalSettings, Message},
};
use std::sync::mpsc::Sender;

use super::{charset::Table, result::PasswordMatch};

pub struct Incremental<'a> {
    target: &'a Vec<u8>,
    range: &'static [u8],
    table: Table,
    algorithm: Box<dyn Algorithm>,
    min_length: usize,
    max_length: usize,
    tx: &'a Sender<Message>,
}

impl<'a> Incremental<'a> {
    pub fn from(settings: &'a ThreadSettings, tx: &'a Sender<Message>) -> Self {
        Self {
            target: settings.target(),
            range: settings.incremental().range(),
            table: settings.incremental().table(),
            algorithm: settings.algorithm(),
            min_length: settings.incremental().min_length(),
            max_length: settings.incremental().max_length(),
            tx,
        }
    }

    pub fn run(&mut self) {
        let mut n = self.min_length;
        let mut counter = 0;
        'runner: loop {
            for c in self.range {
                let mut word = Vec::with_capacity(n);
                word.push(*c);
                if self.calculate(false, n - 1, &mut word) {
                    break 'runner;
                }
            }
            self.tx.send(Message::WordSizeIncreased);
            n += 1;
            if (n > self.max_length) {
                self.tx.send(Message::NoMatch);
                break 'runner;
            }
        }
    }

    fn calculate(&mut self, is_match: bool, length: usize, word: &mut Vec<u8>) -> bool {
        if is_match {
            return true;
        }
        if length == 0 {
            return self.execute_comparison(word);
        }

        self.table.iter().any(|byte| {
            word.push(*byte);
            let result = self.calculate(false, length - 1, word);
            word.pop();
            result
        })
    }

    fn execute_comparison(&mut self, word: &[u8]) -> bool {
        let algorithm = self.algorithm.as_mut();
        algorithm.populate(word);
        algorithm.execute();

        if algorithm.compare(&self.target) {
            let password_match =
                PasswordMatch::from(word, self.algorithm.to_string(), &self.target);
                self.tx.send(Message::PasswordMatch(password_match));
            return true;
        }
        false
    }
}
