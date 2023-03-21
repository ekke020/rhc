use self::manager::Manager;

pub mod setup;
mod manager;

pub type IncrementalSettings = setup::IncrementalSettings;
pub type DictionarySettings = setup::DictionarySettings;

pub enum Message {
    DictionaryProcessed,
    WordSizeIncreased,
    PasswordMatch(Vec<u8>),
    NoMatch,
}

pub fn run(settings: Settings) -> Result<(), CoreError> {
    // TODO: We need some sort of Instant here.
    let thread_settings = setup::ThreadSettings::from(&settings);
    let mut manager = Manager::new(&settings);
    manager.initialize(thread_settings);
    manager.listen();
    Ok(())
}
