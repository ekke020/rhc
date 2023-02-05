mod constants;
mod crack;
mod error;
mod package;
mod setup;
mod spawn;
use self::error::core::CoreError;
use self::package::Package;
use crate::cli::settings::GlobalSettings;

pub type Error = error::core::CoreError;

// TODO: Return a result of the successful crack.
pub fn run(mut settings: GlobalSettings) -> Result<(), CoreError> {
    let package = Package::assemble(&mut settings)?;
    let wordlist = settings.get_wordlist();

    let threads = match wordlist {
        Some(wordlist) => spawn::resource_job(package, wordlist)?,
        None => spawn::brute_force_job(package)?,
    };
    for thread in threads {
        let result = thread.join().unwrap();
        if result.is_some() {
            println!("{}", result.unwrap());
            break;
        }
    }
    Ok(())
}
