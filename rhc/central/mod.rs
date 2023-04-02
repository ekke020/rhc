use crate::{settings::{validator::ProcessedSettings, thread::ThreadSettings}, error::core::CoreError};
use self::manager::Manager;

mod manager;

pub enum Message {
    DictionaryProcessed,
    WordSizeIncreased,
    PasswordMatch(Vec<u8>),
    NoMatch,
}

pub fn run(settings: ProcessedSettings) -> Result<(), CoreError> {
    // TODO: We need some sort of Instant here.
    let thread_settings = ThreadSettings::from(&settings);
    let mut manager = Manager::new(&settings);
    manager.initialize(thread_settings);
    manager.listen();
    Ok(())
}
