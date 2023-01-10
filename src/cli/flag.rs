const INPUT_HELP_MSG: &str = "INPUT TEST";
const TYPE_HELP_MSG: &str = "TYPE TEST";
const LENGTH_HELP_MSG: &str = "LENGTH TEST";
const VERSION: &str = "0.0.1";

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Flag {
    Input,
    Type,
    Length,
    Version,
    Help,
}

impl Flag {
    fn help(&self) -> &str {
        match self {
            Flag::Input => INPUT_HELP_MSG,
            Flag::Type => TYPE_HELP_MSG,
            Flag::Length => LENGTH_HELP_MSG,
            Flag::Version => "",
            Flag::Help => "",
        }
    }
}

pub enum FlagType {
    Option(String),
    Input(String),
    Toggle,
}

#[derive(Debug)]
pub struct FlagInfo {
    flag: Flag,
    input: Option<String>,
    help: bool,
}

impl FlagInfo {
    pub fn from(flag: Flag) -> Self {
        Self {
            flag,
            input: None,
            help: false,
        }
    }

    pub fn set_input(&mut self, input: Option<String>) {
        self.input = input;
    }

    pub fn toggle_help(&mut self) {
        self.help = !self.help;
    }
}