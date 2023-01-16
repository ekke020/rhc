use std::error;
use std::fmt;
use std::process;

const COMMAND_USAGE_ERROR: i32 = 0x40;
const DATA_FORMAT_ERROR: i32 = 0x41;
const INPUT_OUTPUT_ERROR: i32 = 0x4A;

#[derive(Debug)]
enum ArgumentErrorKind {
    NoArgumentSpecified,
    InvalidArgumentPassed,
    MalformedArgument,
    InvalidInput,
    MissingInput,
    NoSuchArgument(String),
}

impl ArgumentErrorKind {
    fn get_exit_code(&self) -> i32 {
        match self {
            ArgumentErrorKind::NoArgumentSpecified => COMMAND_USAGE_ERROR,
            ArgumentErrorKind::InvalidArgumentPassed => COMMAND_USAGE_ERROR,
            ArgumentErrorKind::MalformedArgument => COMMAND_USAGE_ERROR,
            ArgumentErrorKind::InvalidInput => INPUT_OUTPUT_ERROR,
            ArgumentErrorKind::MissingInput => INPUT_OUTPUT_ERROR,
            ArgumentErrorKind::NoSuchArgument(_) => COMMAND_USAGE_ERROR,
        }
    }
    // TODO: Return a String instead for dynamic error messages
    fn get_error_message(&self) -> &str {
        match self {
            ArgumentErrorKind::NoArgumentSpecified => "No argument specified\nUse -h, --help for available options",
            ArgumentErrorKind::InvalidArgumentPassed => "Invalid argument passed\nUse -h, --help for available options",
            ArgumentErrorKind::MalformedArgument => "Argument is malformed\nAll arguments must start with either one or two hyphen('-')\nExample: -h, --help",
            ArgumentErrorKind::InvalidInput => "Invalid input passed after argument\nUse -h, --help for available options",
            ArgumentErrorKind::MissingInput => "Argument requires input\nUse -h, --help for available options",
            ArgumentErrorKind::NoSuchArgument(_) => "No such argument: {}, arg) + \nUse -h, --help for available options",
        }
    }
}

pub const NO_ARGUMENT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::NoArgumentSpecified);
pub const INVALID_ARGUMENT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::InvalidArgumentPassed);
pub const MALFORMED_ARGUMENT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::MalformedArgument);
pub const INVALID_INPUT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::InvalidInput);
pub const MISSING_INPUT_ERROR: ArgumentError = ArgumentError(ArgumentErrorKind::MissingInput);

#[derive(Debug)]
pub struct ArgumentError(ArgumentErrorKind);

impl ArgumentError {
    pub fn no_such_argument(arg: &str) -> Self {
        ArgumentError(ArgumentErrorKind::NoSuchArgument(arg.to_owned()))
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
