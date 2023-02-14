use crate::{cli, core};

#[derive(Debug)]
pub(super) enum Error {
    CliError(cli::Error),
    CoreError(core::Error),
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

impl From<cli::Error> for Error {
    fn from(error: cli::Error) -> Self {
        Error::CliError(error)
    }
}

impl From<core::Error> for Error {
    fn from(error: core::Error) -> Self {
        Error::CoreError(error)
    }
}

pub(super) fn run() -> Result<(), Error> {
    let settings = cli::run()?;
    let result = core::test_brute_force(settings)?;
    Ok(())
}
