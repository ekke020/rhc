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
    current_word_size: usize,
    word_size_counter: usize,
    processed_word_count: usize,
    processed_dictionary_count: usize,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

impl Manager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize(&self, thread_settings: Vec<ThreadSettings>) {
        thread_settings
            .into_iter()
            .for_each(|settings| spawn::job(self.tx.clone(), settings));
    }

    pub fn listen(&mut self, is_quiet: bool) {
        // TODO: Implement methods for the different messages.
        // TODO: Figure out a way to handle quiet mode.
        loop {
            if let Ok(message) = self.rx.recv() {
                match message {
                    Message::DictionaryProcessed => self.processed_dictionary_count += 1,
                    Message::WordSizeIncreased => self.word_size_counter += 1,
                    Message::ProcessedWordCount(count) => self.processed_word_count += count,
                    Message::PasswordMatch(p_match) => todo!(),
                    Message::NoMatch => todo!(),
                }
            } else {
                todo!("Handle the error");
                break;
            }
        }
    }
}

impl Default for Manager {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            current_word_size: Default::default(),
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
    let mut manager = Manager::new();
    manager.initialize(thread_settings);
    manager.listen(settings.quiet());
    Ok(())
}
