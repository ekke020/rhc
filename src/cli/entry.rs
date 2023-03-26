use super::*;
use regex::Regex;
use std::{collections::VecDeque, env};

pub fn produce_settings() -> Result<unvalidated::Settings, ArgumentError> {
    let values = collect_args()?;
    let settings = parse_args(values)?;

    Ok(settings)
}

fn collect_args() -> Result<VecDeque<String>, ArgumentError> {
    let mut args = env::args()
        .into_iter()
        .skip(1)
        .collect::<VecDeque<String>>();
    args.get(0).ok_or(NO_ARGUMENT_ERROR)?;
    Ok(args)
}

fn parse_args(mut args: VecDeque<String>) -> Result<unvalidated::Settings, ArgumentError> {
    let mut settings = unvalidated::Settings::new();

    let mut last_arg = String::from("--help");
    while !args.is_empty() {
        let arg = args.pop_front().unwrap();

        // Call version and exit early if arg is version
        arg.eq("version").then(|| argument::version_and_exit());
        // Call help and exit early if arg is help
        arg.eq("help").then(|| argument::help_and_exit(None));

        is_arg_valid(&arg)?;

        if let Some(f) = flag::get_input(&arg) {
            let input = args.pop_front().ok_or(ArgumentError::missing_input(&arg))?;
            check_help(&input, &arg);
            let setting = f.produce_input_setting(&input)?;
            settings.add_setting(setting);
        } else if let Some(f) = flag::get_toggle(&arg) {
            let setting = f.produce_toggle_setting();
            settings.add_setting(setting);
        } else {
            check_help(&arg, &last_arg);
            return Err(ArgumentError::no_such_argument(&arg));
        }
        last_arg = arg;
    }
    Ok(settings)
}

fn is_arg_valid(value: &str) -> Result<(), ArgumentError> {
    let option = Regex::new(r"^((-[a-zA-Z0-9])|(--[a-zA-Z0-9]{2,})(-[a-zA-Z0-9]{2,})*)$").unwrap();
    if let Some(v) = option.find(value) {
        return Ok(());
    }
    Err(INVALID_ARGUMENT_ERROR)
}

fn check_help(arg: &str, last_arg: &str) {
    if arg.eq("-h") || arg.eq("--help") {
        argument::help_and_exit(Some(last_arg));
    }
}

#[cfg(test)]
mod tests {
    use crate::{algorithm::AlgorithmType, cli::flag::get_input};

    use super::*;

    #[test]
    fn test_collect_args() {
        let result = collect_args();
        assert_eq!(result, Err(NO_ARGUMENT_ERROR));
    }

    #[test]
    fn test_parse_args() -> Result<(), ArgumentError> {
        let args = VecDeque::from([
            "-t",
            "90a3ed9e32b2aaf4c61c410eb925426119e1a9dc53d4286ade99a809",
            "--algorithm",
            "sha2_224",
        ])
        .iter_mut()
        .map(|v| v.to_string())
        .collect();
        let mut result = parse_args(args)?;
        Ok(())
    }

    #[test]
    fn test_is_arg_valid() -> Result<(), ArgumentError> {
        is_arg_valid("--help")?;
        is_arg_valid("-h")?;
        is_arg_valid("--help-help")?;
        Ok(())
    }

    #[test]
    fn test_is_arg_valid_error() {
        let result = is_arg_valid("--awd-c");
        assert_eq!(result, Err(INVALID_ARGUMENT_ERROR));
        let result = is_arg_valid("-wad");
        assert_eq!(result, Err(INVALID_ARGUMENT_ERROR));
        let result = is_arg_valid("--a");
        assert_eq!(result, Err(INVALID_ARGUMENT_ERROR));
        let result = is_arg_valid("--");
        assert_eq!(result, Err(INVALID_ARGUMENT_ERROR));
        let result = is_arg_valid("-");
        assert_eq!(result, Err(INVALID_ARGUMENT_ERROR));
    }
}
