use crate::{error::Error, cli, central};

pub(super) fn run() -> Result<(), Error> {
    let settings = cli::run()?;
    let result = central::run(settings)?;
    Ok(())
}
