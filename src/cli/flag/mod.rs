mod algorithm;
mod help;
mod min_length;
mod max_length;
mod target;
mod verbose;
mod wordlist;
mod mode;
mod charset;
mod thread_count;

use algorithm::Algorithm;
use mode::Mode;
use help::Help;
use min_length::MinLength;
use target::Target;
use verbose::Verbose;
use wordlist::Wordlist;
use phf::phf_map;
use max_length::MaxLength;
use charset::Charset;
use thread_count::ThreadCount;

use super::{
    error::{argument::ArgumentError, flag::FlagError},
    settings::{UnvalidatedSettings, Setting},
};

pub(self) const FLAG_DESCRIPTIONS: [&dyn FlagInfo; 10] =
    [&Help, &Target, &MinLength, &MaxLength, &Verbose, &Wordlist, &Algorithm, &Mode, &Charset, &ThreadCount];

const FLAG_HELP: phf::Map<&str, &dyn FlagHelp> = phf_map! {
    "--help" => &Help,
    "-h" => &Help,
    "--verbose" => &Verbose,
    "-v" => &Verbose,
    "--target" => &Target,
    "-t" => &Target,
    "--charset" => &Charset,
    "-c" => &Charset,
    "--wordlist" => &Wordlist,
    "-w" => &Wordlist,
    "--min-length" => &MinLength,
    "--max-length" => &MaxLength,
    "--algorithm" => &Algorithm,
    "--mode" => &Mode,
    "--thread-count" => &ThreadCount,
};

const FLAG_INPUT: phf::Map<&str, &dyn FlagInput> = phf_map! {
    "--target" => &Target,
    "-t" => &Target,
    "--min-length" => &MinLength,
    "--max-length" => &MaxLength,
    "--wordlist" => &Wordlist,
    "-w" => &Wordlist,
    "--algorithm" => &Algorithm,
    "--mode" => &Mode,
    "--charset" => &Charset,
    "-c" => &Charset,
    "--thread-count" => &ThreadCount,
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
