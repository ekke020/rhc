use crate::{
    algorithm::AlgorithmType, core::charset::CharacterSet, error::argument::ArgumentError,
};

mod dictionary;
mod incremental;
pub mod thread;
pub mod unvalidated;
pub mod validator;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Strategy {
    Dictionary,
    Incremental,
}

impl Strategy {
    pub fn from(value: &str) -> Option<Self> {
        match value {
            "dictionary" => Some(Self::Dictionary),
            "incremental" => Some(Self::Incremental),
            _ => None,
        }
    }
}

pub enum Setting {
    Target(Vec<u8>),
    TargetType(AlgorithmType),
    MinLength(usize),
    MaxLength(usize),
    Quiet(bool),
    Wordlist(Vec<String>),
    ThreadCount(usize),
    Mode(Strategy),
    Charset(CharacterSet),
}
