use std::error;
use std::fmt;
use std::fmt::format;
use std::io;
use std::num::ParseIntError;
use std::process;

use super::COMMAND_USAGE_ERROR;
use super::INPUT_OUTPUT_ERROR;


#[derive(Debug, PartialEq)]
enum ArgumentErrorKind {
    NoArgumentSpecified,
    InvalidArgumentPassed,
    MalformedArgument,
    InvalidInput,
    MissingInput(String),
    NoSuchArgument(String),
    UnsupportedAlgorithm(String),
    UnsupportedCharacterSet(String),
    DetermineAlgorithm,
    UnsupportedMode(String),
    FileEvent(String),
    MalformedHash,
    MissingTargetInput,
    InvalidThreadCount(usize),
    MissingWordList,
    BadLength((usize, usize)),
}

impl ArgumentErrorKind {
    fn get_exit_code(&self) -> i32 {
        match self {
            ArgumentErrorKind::NoArgumentSpecified => COMMAND_USAGE_ERROR,
            ArgumentErrorKind::InvalidArgumentPassed => COMMAND_USAGE_ERROR,
            ArgumentErrorKind::MalformedArgument => COMMAND_USAGE_ERROR,
            ArgumentErrorKind::InvalidInput => INPUT_OUTPUT_ERROR,
            ArgumentErrorKind::MissingInput(_) => INPUT_OUTPUT_ERROR,
            ArgumentErrorKind::NoSuchArgument(_) => COMMAND_USAGE_ERROR,
            ArgumentErrorKind::UnsupportedAlgorithm(_) => INPUT_OUTPUT_ERROR,
            ArgumentErrorKind::UnsupportedCharacterSet(_) => INPUT_OUTPUT_ERROR,
            ArgumentErrorKind::FileEvent(_) => INPUT_OUTPUT_ERROR,
            ArgumentErrorKind::UnsupportedMode(_) => COMMAND_USAGE_ERROR,
            ArgumentErrorKind::MalformedHash => INPUT_OUTPUT_ERROR,
            ArgumentErrorKind::MissingTargetInput => INPUT_OUTPUT_ERROR,
            ArgumentErrorKind::InvalidThreadCount(_) => COMMAND_USAGE_ERROR,
            ArgumentErrorKind::DetermineAlgorithm => 0x01,
            ArgumentErrorKind::MissingWordList => COMMAND_USAGE_ERROR,
            ArgumentErrorKind::BadLength(_) => COMMAND_USAGE_ERROR,
        }
    }

    fn get_error_message(&self) -> String {
        match self {
            ArgumentErrorKind::NoArgumentSpecified => String::from("No argument specified.\nUse -h, --help for available options."),
            ArgumentErrorKind::InvalidArgumentPassed => String::from("Invalid argument passed.\nUse -h, --help for available options."),
            ArgumentErrorKind::MalformedArgument => String::from("Argument is malformed\nAll arguments must start with either one or two hyphen('-').\nExample: -h, --help."),
            ArgumentErrorKind::InvalidInput => String::from("Invalid input passed after argument.\nUse -h, --help for available options."),
            ArgumentErrorKind::MissingInput(arg) => format!("Missing input for argument: {arg}.\nUse {arg} -h, --help for an example."),
            ArgumentErrorKind::NoSuchArgument(arg) => format!("No such argument: {arg}.\nUse -h, --help for available options."),
            ArgumentErrorKind::UnsupportedAlgorithm(arg) => format!("\"{arg}\" is not a suppported algorithm.\nUse --algorithm --help for available algorithms."),
            ArgumentErrorKind::UnsupportedCharacterSet(arg) => format!("\"{arg}\" is not a suppported charset.\nUse --charset --help for available sets."),
            ArgumentErrorKind::DetermineAlgorithm => String::from("Unable to determine algorithm.\nsee --algorithm --help for available options."),
            ArgumentErrorKind::FileEvent(info) => format!("{info},\nUse --wordlist --help for a detailed example."),
            ArgumentErrorKind::UnsupportedMode(arg) => format!("\"{arg}\" is not a suppported mode.\nUse --mode --help for available modes and how to use them."),
            ArgumentErrorKind::MalformedHash => String::from("The input hash is malformed, unable to continue. Validate the hash and try again."),
            ArgumentErrorKind::MissingTargetInput => String::from("No target supplied, unable to run.\nsee --target --help for information."),
            ArgumentErrorKind::InvalidThreadCount(count) => format!("Invalid thread count, not enough threads available.\nsee --threadcount --help for information."),
            ArgumentErrorKind::MissingWordList => String::from("No wordlist supplied, unable to run.\nsee --wordlist --help for information."),
            ArgumentErrorKind::BadLength((min, max)) => format!("Minimum length: \"{min}\" exceeds Maximum length: \"{max}\"."),
        }
    }
}

pub const NO_ARGUMENT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::NoArgumentSpecified);
pub const INVALID_ARGUMENT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::InvalidArgumentPassed);
pub const MALFORMED_ARGUMENT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::MalformedArgument);
pub const INVALID_INPUT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::InvalidInput);
pub const MALFORMED_HASH_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::MalformedHash);
pub const MISSING_TARGET_INPUT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::MissingTargetInput);
pub const DETERMINE_ALGORITHM_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::DetermineAlgorithm);
pub const MISSING_WORD_LIST_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::MissingWordList);
// TODO: Consider changing the name of this error to FlagError
#[derive(Debug, PartialEq)]
pub struct ArgumentError(ArgumentErrorKind);

impl ArgumentError {
    pub fn no_such_argument(arg: &str) -> Self {
        ArgumentError(ArgumentErrorKind::NoSuchArgument(arg.to_owned()))
    }

    pub fn missing_input(arg: &str) -> Self {
        ArgumentError(ArgumentErrorKind::MissingInput(arg.to_owned()))
    }

    pub fn unsupported_algorithm(arg: &str) -> Self {
        ArgumentError(ArgumentErrorKind::UnsupportedAlgorithm(arg.to_owned()))
    }

    pub fn unsupported_character_set(arg: &str) -> Self {
        ArgumentError(ArgumentErrorKind::UnsupportedCharacterSet(arg.to_owned()))
    }

    pub fn unsupported_mode(arg: &str) -> Self {
        ArgumentError(ArgumentErrorKind::UnsupportedMode(arg.to_owned()))
    }

    fn file_event(event: &str) -> Self {
        ArgumentError(ArgumentErrorKind::FileEvent(event.to_owned()))
    }

    pub fn invalid_thread_count(count: usize) -> Self {
        ArgumentError(ArgumentErrorKind::InvalidThreadCount(count))
    }

    pub fn bad_length(min: usize, max: usize) -> Self {
        ArgumentError(ArgumentErrorKind::BadLength((min, max)))
    }
    
    pub fn get_exit_code(&self) -> i32 {
        self.0.get_exit_code()
    }
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.get_error_message())
    }
}

impl error::Error for ArgumentError {}

impl From<io::Error> for ArgumentError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => ArgumentError::file_event("Unable to load the wordlist file (no such file)"),
            io::ErrorKind::PermissionDenied => ArgumentError::file_event(&error.to_string()),
            _ => ArgumentError::file_event("Unable to load file, make sure the path is correct..."),
        }
    }
}

impl From<ParseIntError> for ArgumentError {
    fn from(error: ParseIntError) -> Self {
        MALFORMED_HASH_ERROR
    }
}