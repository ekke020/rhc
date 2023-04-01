use std::{sync::mpsc::{self, Receiver, Sender}, fmt::format, str::from_utf8};
use crate::core::spawn;

use super::*;

pub struct Manager<'a> {
    settings: &'a ProcessedSettings,
    current_word_size: usize,
    word_size_count: usize,
    processed_words: usize,
    processed_dictionary_count: usize,
    no_match_count: usize,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

impl <'a>Manager<'a> {
    pub fn new(settings: &'a ProcessedSettings) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            settings,
            current_word_size: settings.min_length(),
            word_size_count: 0,
            processed_words: 0,
            processed_dictionary_count: 0,
            no_match_count: 0,
            tx,
            rx,
        }
    }

    pub fn initialize(&self, thread_settings: Vec<ThreadSettings>) {
        thread_settings
            .into_iter()
            .for_each(|settings| spawn::job(self.tx.clone(), settings));
    }

    pub fn listen(&mut self) {
        'listener: loop {
            if let Ok(message) = self.rx.recv() {
                match message {
                    Message::DictionaryProcessed => self.handle_dictionary_processed(),
                    Message::WordSizeIncreased => self.handle_word_size_increase(),
                    Message::PasswordMatch(password_match) => {
                        self.handle_password_match(password_match);
                        break 'listener;
                    },
                    Message::NoMatch => {
                        if self.handle_no_match() {
                            break 'listener;
                        }
                    },
                }
            }
        }
    }

    fn handle_dictionary_processed(&mut self) {
        self.processed_dictionary_count += 1
    }

    fn handle_word_size_increase(&mut self) {
        self.word_size_count += 1;
        if self.word_size_count == self.settings.thread_count() {
            println!("Finishied processing words of length: {}", self.current_word_size);
            self.processed_words += self.calculate_solved_words();
            println!("{} words processed", self.processed_words);
            self.current_word_size += 1;
            self.word_size_count = 0;
        }
    }


    fn calculate_solved_words(&self) -> usize {
        let charset_len = self.settings.charset().get_table().len();
        charset_len * charset_len.pow((self.current_word_size - 1) as u32)
    }

    fn handle_password_match(&mut self, password: Vec<u8>) {
        print_match(self.settings, password)
    }

    fn handle_no_match(&mut self) -> bool {
        self.no_match_count += 1;
        if self.no_match_count == self.settings.thread_count() {
            println!("Unable to find a match for: {:?}", self.settings.target());
            println!("Length range: {}-{}", self.settings.min_length(), self.settings.max_length());
            println!("Charset: {}", self.settings.charset());
            return true;
        }
        false
    }
}

fn print_match(settings: &ProcessedSettings, password: Vec<u8>) {
    let target = target_to_lower_hex(settings.target());
    println!("Successfully matched target: {}", target);
    match from_utf8(&password) {
        Ok(value) => {
            println!("Decrypted: {}", value);
            println!("Length: {}", password.len());
        }
        Err(_) => println!("Decrypted bytes: {:?}", password),
    }
    println!("Algorithm: {}", settings.algorithm().generate_algorithm());
}

fn target_to_lower_hex(target: &Vec<u8>) -> String {
    target.iter().map(|dec| format!("{:X}", dec)).collect()
}

fn elapsed_time(elapsed: u64) -> String {
    let seconds = (elapsed % 3600) % 60;
    let minutes = (elapsed % 3600 - seconds) / 60;
    let hours = (elapsed - minutes * 60 + seconds) / 3600;
    format!("H: {}, M: {}, S: {}", hours, minutes, seconds)
}