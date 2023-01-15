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
use self::flag::{FlagInfo, Flag};
use self::settings::gather;

pub fn entrypoint() -> Result<(), ArgumentError>{
    let values = entry::entry()?;
    let mut flags = entry::parse_args(values)?;
   
    flags.iter().for_each(|f| println!("{:?}", f));

    if let Err(e) = handle_flags(&mut flags) {
        println!("{}", e);
        std::process::exit(e.get_exit_code())
    }

    Ok(())
}

fn handle_flags(flags: &mut Vec<Flag>) -> Result<(), FlagError> {

    gather(flags)?;
    Ok(())
}