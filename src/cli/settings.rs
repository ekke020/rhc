use super::{
    errors::flag::FlagError,
    flag::{Flag, FlagInfo},
};

enum Setting {
    HashInput(String),
}

pub struct GlobalSettings {
    settings: Vec<Option<Setting>>,
}

impl GlobalSettings {
    fn new() -> Self {
        GlobalSettings {
            settings: Vec::new(),
        }
    }
}

pub fn gather(flags: Vec<FlagInfo>) -> Result<GlobalSettings, FlagError> {
    let mut settings = GlobalSettings::new();
    for flag in flags {}

    Ok(settings)
}

fn handle_flag(flag: FlagInfo) {
    match flag.get_flag() {
        Flag::Input => {}
        Flag::Type => {}
        Flag::Length => {}
        Flag::Version => {}
        Flag::Help => {}
    };
}
