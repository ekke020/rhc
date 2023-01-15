use std::error;
use std::fmt;
use std::fmt::format;

#[derive(Debug)]
enum FlagErrorKind {
    MissingInputFlag,
    MissingInput(String),
    InvalidInputType(String),
}

impl From<&str> for FlagErrorKind {
    fn from(s: &str) -> Self {
        FlagErrorKind::MissingInput(s.to_string())
    }
}

impl FlagErrorKind {
    fn get_exit_code(&self) -> i32 {
        match self {
            FlagErrorKind::MissingInputFlag => 0x01,
            FlagErrorKind::MissingInput(_) => 0x01,
            FlagErrorKind::InvalidInputType(_) => 0x01,
        }
    }

    fn get_error_message(&self) -> String {
        match self {
            FlagErrorKind::MissingInputFlag => {
                String::from("Mandatory flag --input (-i) not present")
            }
            FlagErrorKind::MissingInput(flag) => format!("Missing input for --{flag}"),
            FlagErrorKind::InvalidInputType(flag) => format!("Invalid input type for --{flag}"),
        }
    }
}

#[derive(Debug)]
pub struct FlagError(FlagErrorKind);

impl FlagError {
    pub fn get_exit_code(&self) -> i32 {
        self.0.get_exit_code()
    }
}

impl FlagError {
    pub fn missing_input_error(input: &str) -> Self {
        FlagError(FlagErrorKind::MissingInput(input.to_string()))
    }

    pub fn invalid_input_type(input: &str) -> Self {
        FlagError(FlagErrorKind::InvalidInputType(input.to_string()))
    }
}

impl fmt::Display for FlagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.get_error_message())
    }
}

impl error::Error for FlagError {}
