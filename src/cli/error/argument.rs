use std::error;
use std::fmt;
use std::io;
use std::process;

const COMMAND_USAGE_ERROR: i32 = 0x40;
const DATA_FORMAT_ERROR: i32 = 0x41;
const INPUT_OUTPUT_ERROR: i32 = 0x4A;

#[derive(Debug, PartialEq)]
enum ArgumentErrorKind {
    NoArgumentSpecified,
    InvalidArgumentPassed,
    MalformedArgument,
    InvalidInput,
    MissingInput(String),
    NoSuchArgument(String),
    UnsupportedAlgorithm(String),
    FileEvent(String),
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
            ArgumentErrorKind::FileEvent(_) => INPUT_OUTPUT_ERROR,
        }
    }

    fn get_error_message(&self) -> String {
        match self {
            ArgumentErrorKind::NoArgumentSpecified => String::from("No argument specified\nUse -h, --help for available options"),
            ArgumentErrorKind::InvalidArgumentPassed => String::from("Invalid argument passed\nUse -h, --help for available options"),
            ArgumentErrorKind::MalformedArgument => String::from("Argument is malformed\nAll arguments must start with either one or two hyphen('-')\nExample: -h, --help"),
            ArgumentErrorKind::InvalidInput => String::from("Invalid input passed after argument\nUse -h, --help for available options"),
            ArgumentErrorKind::MissingInput(arg) => format!("Missing input for argument: {arg}\nUse {arg} -h, --help for an example"),
            ArgumentErrorKind::NoSuchArgument(arg) => format!("No such argument: {arg}\nUse -h, --help for available options"),
            ArgumentErrorKind::UnsupportedAlgorithm(arg) => format!("\"{arg}\" is not a suppported algorithm\nUse --algorithm --help for available algorithms"),
            ArgumentErrorKind::FileEvent(info) => format!("{info}\nUse --wordlist --help for a detailed example"),
        }
    }
}

pub const NO_ARGUMENT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::NoArgumentSpecified);
pub const INVALID_ARGUMENT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::InvalidArgumentPassed);
pub const MALFORMED_ARGUMENT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::MalformedArgument);
pub const INVALID_INPUT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::InvalidInput);

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

    fn file_event(event: &str) -> Self {
        ArgumentError(ArgumentErrorKind::FileEvent(event.to_owned()))
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