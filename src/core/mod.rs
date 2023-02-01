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
pub fn run(mut settings: GlobalSettings) -> Result<(), CoreError>{
    let package = Package::assemble(settings)?;

    Ok(())
}