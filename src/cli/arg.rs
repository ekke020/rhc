use crate::{sha2::Sha224, systems::printer::print::help};
pub struct Arg {
    id: u64,
    required: bool,
    takes_input: bool,
    full_name: String,
    short_name: char,
    help_text: String,
}

impl Arg {
    pub fn new(name: &str) -> Self {
        Arg::default().name(name)
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_short_name(&self) -> char {
        self.short_name
    }

    fn name(mut self, value: &str) -> Self {
        self.id = compute(&value);
        self.full_name = value.to_owned();
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

    pub fn short_name(mut self, short_name: char) -> Self {
        self.short_name = short_name;
        self
    }

    pub fn help_text(mut self, text: &str) -> Self {
        self.help_text = text.to_owned();
        self
    }

    pub fn describe(&self) -> String {
        let mut help_text = String::from("\t\t");
        let mut short_name = String::from("    ");
        help_text.push_str(&self.help_text);
        self.short_name.ne(&'\x00')
            .then(|| short_name = format!(" -{},", self.short_name));
        format!("{} --{} {}", short_name, self.full_name, help_text)
    }
}

impl Default for Arg {
    fn default() -> Self {
        Self {
            id: Default::default(),
            required: false,
            takes_input: false,
            full_name: Default::default(),
            short_name: Default::default(),
            help_text: Default::default(),
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
