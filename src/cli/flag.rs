const VERSION: &str = "0.0.1";

// TODO: Maybe change these so they are connected to the argument info?
#[derive(Debug)]
pub enum Flag {
    Input(FlagInfo),
    Type(FlagInfo),
    Length(FlagInfo),
    Version(FlagInfo),
    Help(FlagInfo),
}
// TODO: Is enum the best way to go? Feels like its very repetitive...
impl Flag {
    // TODO: Change the hardcoded flags to the ones defined in the argument info module
    pub fn from(value: &str) -> Option<Flag> {
        match value {
            "-h" | "--help" => Some(Flag::Help(FlagInfo::new())),
            "-v" | "--version" => Some(Flag::Version(FlagInfo::new())),
            "-i" | "--input" => Some(Flag::Input(FlagInfo::new())),
            "-t" | "--type" => Some(Flag::Type(FlagInfo::new())),
            "-l" | "--length" => Some(Flag::Length(FlagInfo::new())),
            _ => None,
        }
    }

    pub fn set_input(&mut self, input: &str) {
        match self {
            Flag::Input(info) => info.input = Some(input.to_owned()),
            Flag::Type(info) => info.input = Some(input.to_owned()),
            Flag::Length(info) => info.input = Some(input.to_owned()),
            Flag::Version(info) => info.input = Some(input.to_owned()),
            Flag::Help(info) => info.input = Some(input.to_owned()),
        }
    }

    pub fn toggle_help(&mut self) {
        match self {
            Flag::Input(info) => info.toggle_help(),
            Flag::Type(info) => info.toggle_help(),
            Flag::Length(info) => info.toggle_help(),
            Flag::Version(info) => info.toggle_help(),
            Flag::Help(info) => info.toggle_help(),
        }
    }

}

pub enum FlagType {
    Option(String),
    Input(String),
    Help,
}

#[derive(Debug)]
pub struct FlagInfo {
    input: Option<String>,
    help: bool,
}

impl FlagInfo {
    pub fn new() -> Self {
        Self::default()
    }

    fn set_input(&mut self, input: Option<String>) {
        self.input = input;
    }

    fn toggle_help(&mut self) {
        self.help = !self.help;
    }


    pub fn get_input(&mut self) -> Option<String> {
       self.input.take()
    }

    pub fn get_help(&self) -> bool {
        self.help
    }
}

impl Default for FlagInfo {
    fn default() -> Self {
        Self {
            input: Default::default(),
            help: Default::default(),
        }
    }
}
