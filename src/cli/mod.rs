mod argument;
mod entry;
mod error;
mod flags;
mod settings;

use self::settings::GlobalSettings;

pub fn run() -> GlobalSettings {
    let settings = entry::produce_settings().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(e.get_exit_code());
    });
    settings
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
        assert.success().code(0).stdout(predicate::eq(b"rhc version: 0.0.1\n" as &[u8]));
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
