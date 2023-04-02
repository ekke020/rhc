pub mod argument;
pub mod core;

use self::argument::ArgumentError;
use self::core::CoreError;

const NOT_YET_SPECIFIED: i32 = 0x40;
const COMMAND_USAGE_ERROR: i32 = 0x40;
const DATA_FORMAT_ERROR: i32 = 0x41;
const INPUT_OUTPUT_ERROR: i32 = 0x4A;

#[derive(Debug)]
pub(super) enum Error {
    CliError(ArgumentError),
    CoreError(CoreError),
}

impl Error {
    pub fn exit(&self) {
        let exit_code = match self {
            Error::CliError(e) => e.get_exit_code(),
            Error::CoreError(e) => e.get_exit_code(),
        };
        println!("{}", self);
        std::process::exit(exit_code);
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::CliError(e) => write!(f, "{}", e),
            Error::CoreError(e) => write!(f, "{}", e),
        }
    }
}

impl From<ArgumentError> for Error {
    fn from(error: ArgumentError) -> Self {
        Error::CliError(error)
    }
}

impl From<CoreError> for Error {
    fn from(error: CoreError) -> Self {
        Error::CoreError(error)
    }
}