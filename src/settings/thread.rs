use crate::algorithm::Algorithm;

use super::{dictionary::DictionarySettings, incremental::IncrementalSettings, *};

pub struct ThreadSettings {
    target: Vec<u8>,
    thread_count: usize,
    algorithm: AlgorithmType,
    quiet: bool,
    dictionary: Option<DictionarySettings>,
    incremental: IncrementalSettings,
}

impl ThreadSettings {
    pub fn from(settings: &validator::ProcessedSettings) -> Vec<Self> {
        let mut thread_setting = Vec::with_capacity(settings.thread_count());
        for i in 0..settings.thread_count() {
            thread_setting.push(Self::generate(settings, i));
        }
        thread_setting
    }

    fn generate(settings: &validator::ProcessedSettings, count: usize) -> Self {
        Self {
            target: settings.target().to_vec(),
            thread_count: settings.thread_count(),
            algorithm: settings.algorithm().clone(),
            quiet: settings.quiet(),
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

    pub fn quiet(&self) -> bool {
        self.quiet
    }

    pub fn dictionary(&self) -> Option<&DictionarySettings> {
        self.dictionary.as_ref()
    }

    pub fn incremental(&self) -> &IncrementalSettings {
        &self.incremental
    }
}
