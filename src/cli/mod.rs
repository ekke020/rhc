mod errors;
use std::collections::VecDeque;
use std::env;
mod argument;
mod entry;
mod flag;
mod settings;
use argument::info;

use self::errors::argument::ArgumentError;
use self::errors::flag::FlagError;
use self::flag::FlagInfo;
use self::settings::gather;

pub fn entrypoint() -> Result<(), ArgumentError>{
    let values = entry::entry()?;
    let flags = entry::parse_args(values)?;
    flags.iter().for_each(|f| println!("{:?}", f));

    for arg in info::ARGUMENTS {
        println!("{}", argument::describe(arg))
    }

    // settings()?;

    Ok(())
}

fn handle_flags(flags: Vec<FlagInfo>) -> Result<(), FlagError> {

    gather(flags)?;
    Ok(())
}