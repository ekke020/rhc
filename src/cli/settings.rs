use crate::{algorithm::AlgorithmType, core::crack::mode::Mode};

use super::error::argument::ArgumentError;

pub enum Setting {
    HashInput(String),
    HashType(AlgorithmType),
    HashLength(u32),
    Verbose(bool),
    Wordlist(Vec<String>),
    Mode(Mode),
}
#[derive(Debug, Clone)]
pub struct GlobalSettings {
    hash_input: Option<String>,
    hash_type: Option<AlgorithmType>,
    hash_length: Option<u32>,
    wordlist: Option<Vec<String>>,
    verbose: bool,
    mode: Mode,
}
// TODO: Change visibility to super 
impl GlobalSettings {
    pub(crate) fn new() -> Self {
        GlobalSettings {
            hash_input: None,
            hash_type: None,
            hash_length: None,
            wordlist: None,
            verbose: false,
            mode: Mode::Incremental
        }
    }

    pub fn add_setting(&mut self, setting: Setting) {
        match setting {
            Setting::HashInput(value) => self.hash_input = Some(value),
            Setting::HashType(value) => self.hash_type = Some(value),
            Setting::HashLength(value) => self.hash_length = Some(value),
            Setting::Verbose(value) => self.verbose = value,
            Setting::Wordlist(value) => self.wordlist = Some(value),
            Setting::Mode(mode) => self.mode = mode,
        }
    }

    pub fn get_hash_input(&mut self) -> Option<String> {
        self.hash_input.take()
    }

    pub fn get_hash_type(&mut self) -> Option<AlgorithmType> {
        self.hash_type.take()
    }

    pub fn get_hash_length(&mut self) -> Option<u32> {
        self.hash_length.take()
    }

    pub fn get_wordlist(&mut self) -> Option<Vec<String>> {
        self.wordlist.take()
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }
}
