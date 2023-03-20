pub mod setup;
mod spawn;
use crate::{
    cli::Settings,
    core::{error::core::CoreError, result::PasswordMatch},
};
use std::sync::mpsc::{self, Receiver, Sender};

use self::setup::ThreadSettings;

pub type IncrementalSettings = setup::IncrementalSettings;
pub type DictionarySettings = setup::DictionarySettings;

pub enum Message {
    DictionaryProcessed,
    WordSizeIncreased,
    ProcessedWordCount(usize),
    PasswordMatch(PasswordMatch),
    NoMatch,
}

pub struct Manager {
    thread_count: usize,
    current_word_size: usize,
    word_size_counter: usize,
    processed_word_count: usize,
    processed_dictionary_count: usize,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

impl Manager {
    pub fn new(thread_count: usize) -> Self {
        Self {
            thread_count,
            ..Self::default()
        }
    }

    pub fn initialize(&self, thread_settings: Vec<ThreadSettings>) {
        thread_settings
            .into_iter()
            .for_each(|settings| spawn::job(self.tx.clone(), settings));
    }

    pub fn listen(&mut self) {
        // TODO: Implement methods for the different messages.
        // TODO: Figure out a way to handle quiet mode.
        'listener: loop {
            if let Ok(message) = self.rx.recv() {
                match message {
                    Message::DictionaryProcessed => self.handle_dictionary_processed(),
                    Message::WordSizeIncreased => self.handle_word_size_increase(),
                    Message::ProcessedWordCount(count) => self.handle_processed_word_count(),
                    Message::PasswordMatch(password_match) => {
                        self.handle_password_match(password_match);
                        break 'listener;
                    },
                    Message::NoMatch => {
                        // TODO: Maybe change listen to return a result<(), CoreError> instead?
                        // TODO: Then we can throw an error if all threads have returned NoMatch.
                        todo!()
                    },
                }
            } else {
                todo!("Handle the error");
                break;
            }
        }
    }

    fn handle_dictionary_processed(&mut self) {
        self.processed_dictionary_count += 1
    }

    fn handle_word_size_increase(&mut self) {
        self.word_size_counter += 1;
        if self.word_size_counter == self.thread_count {
            println!("Finishied processing words of sice: {}", self.current_word_size);
            self.current_word_size += 1;
            self.word_size_counter = 0;
        }
    }

    fn handle_processed_word_count(&mut self) {
        // self.processed_word_count += count
    }

    fn handle_password_match(&mut self, password_match: PasswordMatch) {
        println!("{}", password_match);

    }

}

impl Default for Manager {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            thread_count: Default::default(),
            current_word_size: 1,
            word_size_counter: Default::default(),
            processed_word_count: Default::default(),
            processed_dictionary_count: Default::default(),
            tx,
            rx,
        }
    }
}

// fn calculate_solved_words() {
//     self.range.len() * self.table.len().pow((n - 1) as u32);
// }

// TODO: Implement a channel central. Each thread will have a connection to the central and will be able to send different information there.
// TODO: For example, whenever a thread is done with its wordlist part it could tell the channel central and if all threads are done the central
// TODO: can communicate this to the user and inform them about the progress.
pub fn run(settings: Settings) -> Result<(), CoreError> {
    // TODO: We need some sort of Instant here.
    let thread_settings = setup::ThreadSettings::from(&settings);
    let mut manager = Manager::new(settings.thread_count());
    manager.initialize(thread_settings);
    manager.listen();
    Ok(())
}
