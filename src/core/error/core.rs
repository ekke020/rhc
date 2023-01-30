use std::error;
use std::fmt;
use std::process;

const NOT_YET_SPECIFIED: i32 = 0x40;

#[derive(Debug, PartialEq)]
enum CoreErrorKind {
    NotYetSpecified,
    InvalidAlgorithm,
    DetermineAlgorithm(String),
    MissingHashInput,
}

impl CoreErrorKind {
    fn get_exit_code(&self) -> i32 {
        match self {
            CoreErrorKind::NotYetSpecified => NOT_YET_SPECIFIED,
            CoreErrorKind::InvalidAlgorithm => NOT_YET_SPECIFIED,
            CoreErrorKind::MissingHashInput => NOT_YET_SPECIFIED,
            CoreErrorKind::DetermineAlgorithm(_) => NOT_YET_SPECIFIED,
        }
    }

    fn get_error_message(&self) -> String {
        match self {
            CoreErrorKind::NotYetSpecified => String::new(),
            CoreErrorKind::InvalidAlgorithm => String::from("Invalid algorithm supplied\nsee --algorithm --help for available options."),
            CoreErrorKind::MissingHashInput => String::from("No hash supplied, unable to run\nsee --password --help for information"),
            CoreErrorKind::DetermineAlgorithm(v) => format!("Unable to determine a possible algorithm from: {v}\nsee --algorithm --help for available options"),
        }
    }
}

pub const MISSING_HASH_INPUT_ERROR: CoreError = CoreError(CoreErrorKind::MissingHashInput);
pub const INVALID_ALGORITHM_ERROR: CoreError = CoreError(CoreErrorKind::InvalidAlgorithm);
// pub const DETERMINE_ALGORITHM_ERROR: CoreError = CoreError(CoreErrorKind::DetermineAlgorithm);

#[derive(Debug, PartialEq)]
pub struct CoreError(CoreErrorKind);

impl CoreError {

    pub fn get_exit_code(&self) -> i32 {
        self.0.get_exit_code()
    }

    pub fn determine_algorithm(value: &Vec<u8>) -> Self {
        let input = std::str::from_utf8(value).unwrap();
        CoreError(CoreErrorKind::DetermineAlgorithm(input.to_owned()))
    }
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.get_error_message())
    }
}

impl error::Error for CoreError {}
