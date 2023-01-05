use std::error;
use std::fmt;
use std::process;

pub const NO_ARGUMENT_ERROR: ArgumentError = ArgumentError::new("No argument specified", true);

pub const INVALID_ARGUMENT_ERROR: ArgumentError =
    ArgumentError::new("Invalid argument passed", true);

pub const MALFORMED_ARGUMENT_ERROR: ArgumentError =
    ArgumentError::new("Argument is malformed\nAll arguments must start with either one or two hyphen('-')\nExample: -h, --help", false);

pub const INVALID_INPUT_ERROR: ArgumentError =
    ArgumentError::new("Invalid input passed after argument", true);

pub const MISSING_INPUT_ERROR: ArgumentError = ArgumentError::new("Argument requires input", true);

#[derive(Debug)]
pub struct ArgumentError<'a> {
    text: &'a str,
    append_help: bool,
}
impl<'a> ArgumentError<'a> {
    const fn new(text: &'a str, append_help: bool) -> Self {
        Self { text, append_help }
    }

    pub fn custom(text: String, append_help: bool) -> Self {
        let text = Box::leak(text.into_boxed_str());
        Self { text, append_help }
    }

    pub fn exit(&self, exit_code: i32) {
        let mut message = self.text.to_owned();
        if self.append_help {
            message.push_str("\nUse -h, --help for available options");
        }
        println!("{}", message);
        process::exit(exit_code);
    }
}

impl<'a> fmt::Display for ArgumentError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl error::Error for ArgumentError<'_> {
    fn description(&self) -> &str {
        self.text
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
