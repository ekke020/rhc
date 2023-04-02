use super::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Settings {
    target: Option<Vec<u8>>,
    target_type: Option<AlgorithmType>,
    min_length: usize,
    max_length: usize,
    wordlist: Option<Vec<String>>,
    quiet: bool,
    modes: HashSet<Strategy>,
    charset: CharacterSet,
    thread_count: usize,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            target: None,
            target_type: None,
            min_length: 1,
            max_length: 999,
            wordlist: None,
            quiet: false,
            modes: HashSet::from([Strategy::Incremental]),
            charset: CharacterSet::Common,
            thread_count: num_cpus::get(),
        }
    }

    pub fn add_setting(&mut self, setting: Setting) {
        match setting {
            Setting::Target(value) => self.target = Some(value),
            Setting::TargetType(value) => self.target_type = Some(value),
            Setting::MinLength(value) => self.min_length = value,
            Setting::MaxLength(value) => self.max_length = value,
            Setting::Quiet(value) => self.quiet = value,
            Setting::Wordlist(value) => self.wordlist = Some(value),
            Setting::ThreadCount(count) => self.thread_count = count,
            Setting::Charset(set) => self.charset = set,
            Setting::Mode(mode) => {
                self.modes.insert(mode);
            }
        }
    }

    pub fn target(&self) -> Option<&Vec<u8>> {
        self.target.as_ref()
    }

    pub fn target_type(&self) -> Option<&AlgorithmType> {
        self.target_type.as_ref()
    }

    pub fn min_length(&self) -> usize {
        self.min_length
    }

    pub fn max_length(&self) -> usize {
        self.max_length
    }

    pub fn wordlist(&self) -> Option<&Vec<String>> {
        self.wordlist.as_ref()
    }

    pub fn quiet(&self) -> bool {
        self.quiet
    }

    pub fn modes(&mut self) -> &mut HashSet<Strategy> {
        &mut self.modes
    }

    pub fn charset(&self) -> &CharacterSet {
        &self.charset
    }

    pub fn thread_count(&self) -> usize {
        self.thread_count
    }
}

impl Into<validator::ProcessedSettings> for Settings {
    fn into(self) -> validator::ProcessedSettings {
        validator::ProcessedSettings {
            target: self.target.unwrap(),
            thread_count: self.thread_count,
            algorithm: self.target_type.unwrap(),
            quiet: self.quiet,
            modes: self.modes,
            wordlist: self.wordlist.unwrap_or(vec![]),
            min_length: self.min_length,
            max_length: self.max_length,
            charset: self.charset,
        }
    }
}
