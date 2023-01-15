use super::{
    argument::describe,
    errors::flag::FlagError,
    flag::{Flag, FlagInfo},
};
use std::collections::HashMap;

enum Setting {
    HashInput(String),
    HashType(String),
    HashLenght(u32),
}

pub struct GlobalSettings {
    hash_input: Option<Setting>,
    hash_type: Option<Setting>,
    hash_length: Option<Setting>,
}

impl GlobalSettings {
    fn new() -> Self {
        GlobalSettings {
            hash_input: None,
            hash_type: None,
            hash_length: None,
        }
    }

    fn add_setting(&mut self, setting: Setting) {
        match setting {
            Setting::HashInput(_) => self.hash_input = Some(setting),
            Setting::HashType(_) => self.hash_type = Some(setting),
            Setting::HashLenght(_) => self.hash_length = Some(setting),
        }
    }
}

pub fn gather(flags: &mut Vec<Flag>) -> Result<GlobalSettings, FlagError> {
    let mut settings = GlobalSettings::new();
    for flag in flags {
        handle_flag(flag, &mut settings)?;
    }

    Ok(settings)
}

fn handle_flag(flag: &mut Flag, settings: &mut GlobalSettings) -> Result<(), FlagError> {
    match flag {
        Flag::Input(info) => {
            // TODO: Change the hardcoded value of "input" to the const value of argumentinfo
            let input = info
                .get_input()
                .ok_or(FlagError::missing_input_error("input"))?;
            settings.hash_input = Some(Setting::HashInput(input));
        }
        Flag::Type(info) => {
            // TODO: Change the hardcoded value of "type" to the const value of argumentinfo
            let input = info
                .get_input()
                .ok_or(FlagError::missing_input_error("type"))?;
            settings.hash_type = Some(Setting::HashType(input));
        }
        Flag::Length(info) => {
            // TODO: Change the hardcoded value of "length" to the const value of argumentinfo
            let input = info
                .get_input()
                .ok_or(FlagError::missing_input_error("length"))?;
            let test = input
                .parse::<u32>()
                .ok()
                .ok_or(FlagError::invalid_input_type("length"))?;
            settings.hash_length = Some(Setting::HashLenght(test));
        }
        Flag::Version(info) => {
            info.get_help().ne(&true).then(|| version());
        }
        Flag::Help(info) => {
            info.get_help().then(|| help());
        }
    };  
    Ok(())
}

use super::argument::info::ARGUMENTS;
fn help() {
    println!("usage: rhc [OPTION]...");
    ARGUMENTS
        .iter()
        .for_each(|arg| println!("{}", describe(arg)));
    std::process::exit(0);
}

// TODO: Move this value to the correct place
pub const VERSION: &str = "0.0.1";
fn version() {
    println!("Version: {}", VERSION);
    std::process::exit(0);
}
