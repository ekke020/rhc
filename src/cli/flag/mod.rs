mod algorithm;
mod help;
mod length;
mod password;
mod verbose;
mod wordlist;
mod mode;

use algorithm::Algorithm;
use mode::Mode;
use help::Help;
use length::Length;
use password::Password;
use verbose::Verbose;
use wordlist::Wordlist;
use phf::phf_map;


use super::{
    error::{argument::ArgumentError, flag::FlagError},
    settings::{InputOptions, Setting},
};

pub(self) const FLAG_DESCRIPTIONS: [&dyn FlagInfo; 7] =
    [&Help, &Password, &Length, &Verbose, &Wordlist, &Algorithm, &Mode];

const FLAG_HELP: phf::Map<&str, &dyn FlagHelp> = phf_map! {
    "--help" => &Help,
    "-h" => &Help,
    "--verbose" => &Verbose,
    "-v" => &Verbose,
    "--password" => &Password,
    "-p" => &Password,
    "--length" => &Length,
    "-l" => &Length,
    "--wordlist" => &Wordlist,
    "-w" => &Wordlist,
    "--algorithm" => &Algorithm,
    "--mode" => &Mode,
};

const FLAG_INPUT: phf::Map<&str, &dyn FlagInput> = phf_map! {
    "--password" => &Password,
    "-p" => &Password,
    "--length" => &Length,
    "-l" => &Length,
    "--wordlist" => &Wordlist,
    "-w" => &Wordlist,
    "--algorithm" => &Algorithm,
    "--mode" => &Mode,
};

const FLAG_TOGGLE: phf::Map<&str, &dyn FlagToggle> = phf_map! {
    "--verbose" => &Verbose,
    "-v" => &Verbose,
};

pub trait FlagInfo {
    fn describe(&self) -> String;
}
pub trait FlagHelp {
    fn help(&self) -> String;
}

pub trait FlagInput {
    fn produce_input_setting(&self, value: &str) -> Result<Setting, ArgumentError>;
}

pub trait FlagToggle {
    fn produce_toggle_setting(&self) -> Setting;
}

pub fn get_help(flag: &str) -> Result<&&dyn FlagHelp, ArgumentError> {
    FLAG_HELP
        .get(flag)
        .ok_or(ArgumentError::no_such_argument(flag))
}

pub fn get_input(flag: &str) -> Option<&&dyn FlagInput> {
    FLAG_INPUT.get(flag)
}

pub fn get_toggle(flag: &str) -> Option<&&dyn FlagToggle> {
    FLAG_TOGGLE.get(flag)
}
