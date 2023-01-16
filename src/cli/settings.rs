pub enum Setting {
    HashInput(String),
    HashType(String),
    HashLenght(u32),
    Verbose(bool),
}

pub struct GlobalSettings {
    hash_input: Option<Setting>,
    hash_type: Option<Setting>,
    hash_length: Option<Setting>,
    verbose: Option<Setting>,
}

impl GlobalSettings {
    pub(super) fn new() -> Self {
        GlobalSettings {
            hash_input: None,
            hash_type: None,
            hash_length: None,
            verbose: None,
        }
    }

    pub fn add_setting(&mut self, setting: Setting) {
        match setting {
            Setting::HashInput(_) => self.hash_input = Some(setting),
            Setting::HashType(_) => self.hash_type = Some(setting),
            Setting::HashLenght(_) => self.hash_length = Some(setting),
            Setting::Verbose(_) => self.verbose = Some(setting),
        }
    }

    pub fn get_hash_input(&mut self) -> Option<Setting> {
        self.hash_input.take()
    }

    pub fn get_hash_type(&mut self) -> Option<Setting> {
        self.hash_type.take()
    }

    pub fn get_hash_length(&mut self) -> Option<Setting> {
        self.hash_length.take()
    }

    pub fn is_verbose(&mut self) -> Option<Setting> {
        self.verbose.take()
    }
}
