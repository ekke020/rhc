use crate::algorithm::{AlgorithmType, Algorithm};
use crate::cli::Settings;
// TODO: Replace package with a SettingsValidator 
// TODO: After validating, prepare settings for respective thread.
// TODO: ThreadSettings? Include symbols slice?
// TODO: Return an array containing the prepared settings and distribute across threads.
// TODO: Tear down the threads after a sucessfull crack
// TODO: Present output 

struct IncrementalSettings {
    max_length: usize,
    min_length: usize,
    range: &'static [u8],
}

struct DictionarySettings<'a> {
    wordlist: &'a [String],
}

pub struct ThreadSettings<'a> {
    target: &'a Vec<u8>,
    thread_count: usize,
    algorithm: Box<dyn Algorithm>,
    verbose: bool,
    dictionary: Option<DictionarySettings<'a>>,
    incremental: IncrementalSettings,
}


impl <'a>ThreadSettings<'a> {

    fn from(settings: &Settings) -> Self {
        Self {
            target: settings.target(),
            thread_count: settings.thread_count(),
            algorithm: settings.algorithm().generate_algorithm(),
            verbose: settings.verbose(),
        }
    }

}

pub fn generate_thread_settings(settings: &Settings) -> Vec<ThreadSettings> {
    todo!()
}