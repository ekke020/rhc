mod argument;
mod entry;
mod error;
mod flag;
mod settings;

use self::settings::GlobalSettings;

pub fn run() -> GlobalSettings {
    let flags = entry::produce_flags().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(e.get_exit_code());
    });
    let settings = settings::produce_settings(flags).unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(e.get_exit_code())
    });
    settings
}
