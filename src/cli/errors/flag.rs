use std::fmt;
use std::error;

#[derive(Debug)]
enum FlagErrorKind {
    MissingInputFlagError,

}

impl FlagErrorKind {
    fn get_exit_code(&self) -> i32 {
        match self {
            FlagErrorKind::MissingInputFlagError => 0x01,
        }
    }

    fn get_error_message(&self) -> &str {
        match self {
            FlagErrorKind::MissingInputFlagError => "Mandatory flags not present\n-i is required",
        }
    }
}

#[derive(Debug)]
pub struct FlagError(FlagErrorKind);

impl fmt::Display for FlagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.get_error_message())
    }
}

impl error::Error for FlagError {}