mod setup;
mod spawn;

use std::sync::mpsc;
use crate::{cli::Settings, core::error::core::CoreError};

pub type IncrementalSettings = setup::IncrementalSettings;
pub type DictionarySettings = setup::DictionarySettings;

enum Message {
    StringMsg(String),
    IntMsg(i32),
}

pub struct Manager {}

impl Manager {
    pub fn initialize() {}
}

// TODO: Implement a channel central. Each thread will have a connection to the central and will be able to send different information there.
// TODO: For example, whenever a thread is done with its wordlist part it could tell the channel central and if all threads are done the central
// TODO: can communicate this to the user and inform them about the progress.
pub fn run(settings: Settings) -> Result<(), CoreError> {
    // TODO: We need some sort of Instant here.
    let thread_settings = setup::ThreadSettings::from(&settings);
    let (tx, rx) = mpsc::channel();
    thread_settings
        .into_iter()
        .for_each(|settings| spawn::job(tx.clone(), settings));
    // TODO: The passwordmatch should not be the final result that the user sees...
    let value = rx.recv().unwrap();
    match value {
        Some(pm) => println!("{pm}"),
        None => todo!(),
    }
    Ok(())
}
