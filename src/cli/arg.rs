use crate::{sha2::Sha224, systems::printer::print::help};
#[derive(Debug)]
pub struct Arg {
    id: u64,
    required: bool,
    takes_input: bool,
    name: String,
    shorthand: Option<char>,
    help: Option<String>,
    long_help: Option<String>,
}

impl Arg {
    pub fn new(name: &str) -> Self {
        Arg::default().name(name)
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_shorthand(&self) -> Option<&char> {
        self.shorthand.as_ref()
    }
    
    fn name(mut self, value: &str) -> Self {
        self.id = compute(&value);
        self.name = value.to_owned();
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn takes_input(mut self, input: bool) -> Self {
        self.takes_input = input;
        self
    }

    pub fn short_name(mut self, shorhand: char) -> Self {
        self.shorthand = Some(shorhand);
        self
    }

    pub fn help(mut self, text: &str) -> Self {
        self.help = Some(text.to_owned());
        self
    }

    pub fn get_help(&self) -> Option<&String> {
        self.help.as_ref() 
    }

    pub fn describe(&self) -> String {
        let mut help_text = String::from("\t\t");
        let mut short_name = String::from("    ");
        if let Some(text) = self.get_help() {
            help_text.push_str(text);
        }
        if let Some(shorthand) = self.get_shorthand() {
            short_name = format!(" -{},", shorthand);
        };
        format!("{} --{} {}", short_name, self.name, help_text)
    }
}

impl Default for Arg {
    fn default() -> Self {
        Self {
            id: Default::default(),
            required: false,
            takes_input: false,
            name: Default::default(),
            shorthand: Default::default(),
            help: Default::default(),
            long_help: Default::default(),
        }
    }
}

fn compute(value: impl AsRef<[u8]>) -> u64 {
    let input: Vec<u64> = value.as_ref().iter().map(|b| u64::from(*b)).collect();
    let mulp: u64 = 2654435789;
    let mut mix: u64 = 104395301;

    for byte in input {
        mix += (byte * mulp) ^ (mix >> 12);
    }
    mix ^ (mix << 42)
}
