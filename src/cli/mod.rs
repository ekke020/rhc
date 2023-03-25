mod argument;
mod entry;
mod error;
mod flag;

use crate::error::argument::*;
use crate::settings::*;

pub fn run() -> Result<validator::ProcessedSettings, ArgumentError> {
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
        let version = format!("rhc version: {}\n", env!("CARGO_PKG_VERSION"));
        let mut cmd = Command::cargo_bin("rhc").unwrap();
        let assert = cmd.arg("version").assert();
        assert
            .success()
            .code(0)
            .stdout(predicate::eq(version.as_bytes()));
    }

    #[test]
    fn test_run_missing_input() {
        let mut cmd = Command::cargo_bin("rhc").unwrap();
        let assert = cmd.arg("-t").assert();
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
