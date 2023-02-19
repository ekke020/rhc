mod argument;
mod entry;
mod error;
mod flag;
pub mod settings;
use self::{settings::{UnvalidatedSettings, validator::{self, ProcessedSettings}}, error::argument::ArgumentError};

pub type Error = error::argument::ArgumentError;
pub type Settings = ProcessedSettings;

pub fn run() -> Result<ProcessedSettings, ArgumentError> {
    let raw_settings = entry::produce_settings()?;
    let validated = validator::validate(raw_settings)?;
    Ok(validated)
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_run() {
        let mut cmd = Command::cargo_bin("rhc").unwrap();
        let assert = cmd.arg("--help").assert();
        assert.success().code(0);
    }

    #[test]
    fn test_run_help() {
        let mut cmd = Command::cargo_bin("rhc").unwrap();
        let assert = cmd.arg("help").assert();
        assert.success().code(0);
    }

    #[test]
    fn test_run_version() {
        let mut cmd = Command::cargo_bin("rhc").unwrap();
        let assert = cmd.arg("version").assert();
        assert
            .success()
            .code(0)
            .stdout(predicate::eq(b"rhc version: 0.0.1\n" as &[u8]));
    }

    #[test]
    fn test_run_missing_input() {
        let mut cmd = Command::cargo_bin("rhc").unwrap();
        let assert = cmd.arg("-p").assert();
        assert.failure().code(0x4A);
    }
    #[test]
    fn test_run_invalid_argument() {
        let mut cmd = Command::cargo_bin("rhc").unwrap();
        let assert = cmd.env_clear().arg("--").assert();
        assert.failure().code(0x40);
    }
    #[test]
    fn test_run_no_argument() {
        let mut cmd = Command::cargo_bin("rhc").unwrap();
        let assert = cmd.env_clear().assert();
        assert.failure().code(0x40);
    }
}
