use std::process;
use std::fmt;
use std::error;

pub const NO_ARGUMENT_ERROR: ArgumentError =
    ArgumentError::new("No argument specified\nUse -h, --help for available options");

pub const INVALID_ARGUMENT_ERROR: ArgumentError =
    ArgumentError::new("Invalid argument passed\nUse -h, --help for available options");

pub const MALFORMED_ARGUMENT_ERROR: ArgumentError =
    ArgumentError::new("Argument is malformed\nAll arguments must start with either one or two hyphen('-')\nExample: -h, --help");

pub const INVALID_INPUT_ERROR: ArgumentError =
    ArgumentError::new("Invalid input passed after argument\nUse -h, --help for available options");

pub const MISSING_INPUT_ERROR: ArgumentError =
    ArgumentError::new("Argument requires input\nUse -h, --help for available options");

#[derive(Debug)]
pub struct ArgumentError<'a>(&'a str);

impl<'a> ArgumentError<'a> {
    pub const fn new(text: &'a str) -> Self {
        ArgumentError(text)
    }

    pub fn exit(&self, exit_code: i32) {
        println!("{}", self.0);
        process::exit(exit_code);
    }
}

impl<'a> fmt::Display for ArgumentError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for ArgumentError<'_> {

    fn description(&self) -> &str {
        self.0
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        
        None
    }

}
