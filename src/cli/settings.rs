use crate::core::setup::Package;

use super::error::argument::ArgumentError;

#[derive(Debug, Clone, PartialEq)]
pub enum AlgorithmType {
    Sha2_224,
    Sha2_256,
    Sha2_384,
    Sha2_512,
    Sha2_512_224,
    Sha2_512_256,
}

impl AlgorithmType {
    pub fn from(value: &str) -> Result<Self, ArgumentError> {
        match value {
            "sha2_224" => Ok(AlgorithmType::Sha2_224),
            "sha2_256" => Ok(AlgorithmType::Sha2_256), 
            "sha2_384" => Ok(AlgorithmType::Sha2_384),
            "sha2_512" => Ok(AlgorithmType::Sha2_512), 
            "sha2_512_224" => Ok(AlgorithmType::Sha2_512_224), 
            "sha2_512_256" => Ok(AlgorithmType::Sha2_512_256),
            _ => Err(ArgumentError::unsupported_algorithm(value))
        }
    }
}

pub enum Setting {
    HashInput(String),
    HashType(AlgorithmType),
    HashLength(u32),
    Verbose(bool),
}
#[derive(Debug, Clone)]
pub struct GlobalSettings {
    hash_input: Option<String>,
    hash_type: Option<AlgorithmType>,
    hash_length: Option<u32>,
    verbose: bool,
}
// TODO: Change visibility to super 
impl GlobalSettings {
    pub(crate) fn new() -> Self {
        GlobalSettings {
            hash_input: None,
            hash_type: None,
            hash_length: None,
            verbose: false,
        }
    }

    pub fn add_setting(&mut self, setting: Setting) {
        match setting {
            Setting::HashInput(value) => self.hash_input = Some(value),
            Setting::HashType(value) => self.hash_type = Some(value),
            Setting::HashLength(value) => self.hash_length = Some(value),
            Setting::Verbose(value) => self.verbose = value,
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

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }
}
