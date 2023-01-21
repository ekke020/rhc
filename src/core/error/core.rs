use std::error;
use std::fmt;
use std::process;

const NOT_YET_SPECIFIED: i32 = 0x40;

#[derive(Debug, PartialEq)]
enum CoreErrorKind {
    NotYetSpecified,
}

impl CoreErrorKind {
    fn get_exit_code(&self) -> i32 {
        match self {
            CoreErrorKind::NotYetSpecified => NOT_YET_SPECIFIED,
        }
    }

    fn get_error_message(&self) -> String {
        match self {
            CoreErrorKind::NotYetSpecified => String::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CoreError(CoreErrorKind);

impl CoreError {

    pub fn get_exit_code(&self) -> i32 {
        self.0.get_exit_code()
    }
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.get_error_message())
    }
}

impl error::Error for CoreError {}
