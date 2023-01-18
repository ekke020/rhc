pub enum Setting {
    HashInput(String),
    HashType(String),
    HashLenght(u32),
    Verbose(bool),
}

pub struct GlobalSettings {
    hash_input: Option<String>,
    hash_type: Option<String>,
    hash_length: Option<u32>,
    verbose: bool,
}

impl GlobalSettings {
    pub(super) fn new() -> Self {
        GlobalSettings {
            hash_input: None,
            hash_type: None,
            hash_length: None,
            verbose: false,
        }
    }

    pub fn add_setting(&mut self, setting: Setting) {
        match setting {
            Setting::HashInput(value) => self.hash_input = Some(value),
            Setting::HashType(value) => self.hash_type = Some(value),
            Setting::HashLenght(value) => self.hash_length = Some(value),
            Setting::Verbose(value) => self.verbose = value,
        }
    }

    pub fn get_hash_input(&mut self) -> Option<String> {
        self.hash_input.take()
    }

    pub fn get_hash_type(&mut self) -> Option<String> {
        self.hash_type.take()
    }

    pub fn get_hash_length(&mut self) -> Option<u32> {
        self.hash_length.take()
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }
}
