use super::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const SHORTHAND: char = 'w';
const NAME: &str = "wordlist";
const SHORT_HELP: &str = "The filepath to the wordlist";
const LONG_HELP: &str = "Flag: -w | --wordlist
Details:
    input type: The filepath to the wordlist
Description:
    The --wordlist flag is used to specify the file containing the 
    list of words to use as input for cracking the hash. 
    The file must be formatted as a wordlist, with each word 
    separated by a newline. When the wordlist file is specified, 
    the cracker will iterate through the words in the list until 
    a match with the input hash is found or all words have been tried.
Example: 
    rhc [OPTIONS]... -f ~/path/to/file/words.txt
Example wordlist:
    # words.txt
    password1
    password2
    password3
    ...
";

pub(super) struct Wordlist;

impl FlagInfo for Wordlist {
    fn describe(&self) -> String {
        format!("-{SHORTHAND}, --{NAME} \t\t{SHORT_HELP}")
    }
}

impl FlagHelp for Wordlist {
    fn help(&self) -> String {
        LONG_HELP.to_owned()
    }
}

impl FlagInput for Wordlist {
    fn produce_input_setting(&self, path: &str) -> Result<Setting, ArgumentError> {
        let words = read_word_file(path)?;
        Ok(Setting::Wordlist(words))
    }
}

fn read_word_file(path: &str) -> Result<Vec<String>, ArgumentError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    Ok(lines)
}
