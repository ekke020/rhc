use std::error;
use std::fmt;
use std::num::ParseIntError;
use std::process;

const NOT_YET_SPECIFIED: i32 = 0x40;

#[derive(Debug, PartialEq)]
enum CoreErrorKind {
    NotYetSpecified,
    InvalidAlgorithm,
    DetermineAlgorithm(String),
    MissingHashInput,
    MalformedHash,
    ByteConversion(Vec<u8>, String),
}

impl CoreErrorKind {
    fn get_exit_code(&self) -> i32 {
        match self {
            CoreErrorKind::NotYetSpecified => NOT_YET_SPECIFIED,
            CoreErrorKind::InvalidAlgorithm => NOT_YET_SPECIFIED,
            CoreErrorKind::MissingHashInput => NOT_YET_SPECIFIED,
            CoreErrorKind::DetermineAlgorithm(_) => NOT_YET_SPECIFIED,
            CoreErrorKind::MalformedHash => NOT_YET_SPECIFIED,
            CoreErrorKind::ByteConversion(_, _) => NOT_YET_SPECIFIED,
        }
    }

    fn get_error_message(&self) -> String {
        match self {
            CoreErrorKind::NotYetSpecified => String::new(),
            CoreErrorKind::InvalidAlgorithm => String::from("Invalid algorithm supplied\nsee --algorithm --help for available options."),
            CoreErrorKind::MissingHashInput => String::from("No hash supplied, unable to run\nsee --password --help for information"),
            CoreErrorKind::DetermineAlgorithm(v) => format!("Unable to determine a possible algorithm from: {v}\nsee --algorithm --help for available options"),
            CoreErrorKind::MalformedHash => String::from("The input hash is malformed, unable to continue. Validate the hash and try again."),
            CoreErrorKind::ByteConversion(bytes, hash) => format!("There was a problem converting the bytes: {:?} to a string\nThe bytes equal the hashed value: {}", bytes, hash),
        }
    }
}

pub const MISSING_HASH_INPUT_ERROR: CoreError = CoreError(CoreErrorKind::MissingHashInput);
pub const INVALID_ALGORITHM_ERROR: CoreError = CoreError(CoreErrorKind::InvalidAlgorithm);
pub const MALFORMED_HASH_ERROR: CoreError = CoreError(CoreErrorKind::MalformedHash);

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

    pub fn byte_conversion(bytes: &Vec<u8>, hash: &str) -> Self{
        CoreError(CoreErrorKind::ByteConversion(bytes.to_vec(), hash.to_owned()))
    }
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.get_error_message())
    }
}

impl error::Error for CoreError {}

impl From<ParseIntError> for CoreError {
    fn from(error: ParseIntError) -> Self {
        MALFORMED_HASH_ERROR
    }
}
