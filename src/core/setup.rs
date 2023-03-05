use std::collections::HashSet;

use crate::algorithm::{AlgorithmType, Algorithm};
use crate::cli::{Settings, Incremental, Dictionary};

use super::charset::{NO_SPECIAL_TABLE, Table};

pub struct IncrementalSettings {
    max_length: usize,
    min_length: usize,
    range: &'static [u8],
    table: Table,
}

impl IncrementalSettings {
    fn from(values: Incremental, count: usize) -> Self {
        Self {
            max_length: values.max_length(),
            min_length: values.min_length(),
            range: Self::get_range(count, values.thread_count()),
            // TODO: Table should not be hardcoded.
            table: NO_SPECIAL_TABLE,
        }
    }

    fn get_range(count: usize, num_cores: usize) -> &'static [u8] {
        let chunk_size = NO_SPECIAL_TABLE.len() / num_cores;

        let start = chunk_size * count;
        let end = if count == num_cores {
            NO_SPECIAL_TABLE.len()
        } else {
            chunk_size * (count + 1)
        };
        &NO_SPECIAL_TABLE[start..end]
    }

    pub fn max_length(&self) -> usize {
        self.max_length
    }

    pub fn min_length(&self) -> usize {
        self.min_length
    }

    pub fn range(&self) -> &'static [u8] {
        self.range
    }

    pub fn table(&self) -> &'static [u8] {
        self.table
    }
}

pub struct DictionarySettings {
    wordlist: Vec<String>,
}

impl <'a>DictionarySettings {
    fn from(values: Option<Dictionary<'a>>, count: usize) -> Option<Self> {
        match values {
            Some(values) => {
                let chunk_size = values.wordlist().len() / values.thread_count();
                let words = values.wordlist().chunks(chunk_size).nth(count).unwrap();
                Some(DictionarySettings{wordlist: words.to_vec()})
            },
            None => None,
        }
    }

    pub fn wordlist(&self) -> &[String] {
        &self.wordlist
    }
}

pub struct ThreadSettings {
    target: Vec<u8>,
    thread_count: usize,
    algorithm: AlgorithmType,
    verbose: bool,
    dictionary: Option<DictionarySettings>,
    incremental: IncrementalSettings,
}


impl ThreadSettings {

    pub fn from(settings: &Settings) -> Vec<Self> {
        let mut thread_setting = Vec::with_capacity(settings.thread_count());
        for i in 0..settings.thread_count() {
            thread_setting.push(Self::generate(settings, i));
        }
        thread_setting
    }

    fn generate(settings: &Settings, count: usize) -> Self {
        Self {
            target: settings.target().to_vec(),
            thread_count: settings.thread_count(),
            algorithm: settings.algorithm().clone(),
            verbose: settings.verbose(),
            dictionary: DictionarySettings::from(settings.dictionary_values(), count),
            incremental: IncrementalSettings::from(settings.incremental_values(), count),
        }
    }

    pub fn target(&self) -> &Vec<u8> {
        &self.target
    }

    pub fn algorithm(&self) -> Box<dyn Algorithm> {
        self.algorithm.generate_algorithm()
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn dictionary(&self) -> Option<&DictionarySettings> {
        self.dictionary.as_ref()
    }

    pub fn incremental(&self) -> &IncrementalSettings {
        &self.incremental
    }
}
