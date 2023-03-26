use std::{str::from_utf8, sync::mpsc::Sender};

use crate::{
    algorithm::{self, Algorithm},
    central::Message,
    settings::thread::ThreadSettings
};

pub struct Dictionary<'a> {
    target: &'a Vec<u8>,
    wordlist: &'a [String],
    algorithm: Box<dyn Algorithm>,
    quiet: bool,
    tx: &'a Sender<Message>,
}

impl<'a> Dictionary<'a> {
    pub fn from(settings: &'a ThreadSettings, tx: &'a Sender<Message>) -> Self {
        Self {
            target: settings.target(),
            wordlist: settings.dictionary().unwrap().wordlist(),
            algorithm: settings.algorithm(),
            quiet: settings.quiet(),
            tx,
        }
    }

    pub fn run(&mut self) {
        self.wordlist
            .iter()
            .any(|word| self.execute_comparison(&word.as_bytes()));

        if !self.quiet {
            self.tx.send(Message::DictionaryProcessed);
        }
    }

    fn execute_comparison(&mut self, word: &[u8]) -> bool {
        let algorithm = self.algorithm.as_mut();
        algorithm.populate(word);
        algorithm.execute();

        if algorithm.compare(&self.target) {
            self.tx.send(Message::PasswordMatch(word.to_vec()));
            return true;
        }
        false
    }
}
