mod entry;
mod error;
mod settings;
mod argument;
mod flags;

use self::settings::GlobalSettings;

pub fn run() -> GlobalSettings {
    let settings = entry::produce_settings().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(e.get_exit_code());
    });
    settings
}
