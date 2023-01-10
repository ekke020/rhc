use std::error;
use std::fmt;
use std::process;

pub const NO_ARGUMENT_ERROR: ArgumentError = ArgumentError {
    text: "No argument specified",
    append_help: true,
};

pub const INVALID_ARGUMENT_ERROR: ArgumentError = ArgumentError {
    text: "Invalid argument passed",
    append_help: true,
};

pub const MALFORMED_ARGUMENT_ERROR: ArgumentError = ArgumentError {
    text: "Argument is malformed\nAll arguments must start with either one or two hyphen('-')\nExample: -h, --help", 
    append_help: false,
};

pub const INVALID_INPUT_ERROR: ArgumentError = ArgumentError {
    text: "Invalid input passed after argument",
    append_help: true,
};

pub const MISSING_INPUT_ERROR: ArgumentError = ArgumentError {
    text: "Argument requires input",
    append_help: true,
};

#[derive(Debug)]
pub struct ArgumentError {
    text: &'static str,
    append_help: bool,
}
impl ArgumentError {
    // const fn new(text: &str, append_help: bool) -> Self {
    //     Self { text, append_help }
    // }

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

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl error::Error for ArgumentError {}

#[derive(Debug)]
pub struct MyError {
    msg: &'static str,
}
