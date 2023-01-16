use std::{collections::VecDeque, env};

use regex::Regex;

use super::{
    argument::{help::print_help, version::print_version},
    error::argument::{
        ArgumentError, INVALID_ARGUMENT_ERROR, MISSING_INPUT_ERROR, NO_ARGUMENT_ERROR,
    },
    flags,
    settings::GlobalSettings,
};

pub fn produce_settings() -> Result<GlobalSettings, ArgumentError> {
    let values = collect_args()?;
    let settings = parse_args(values)?;

    Ok(settings)
}

fn collect_args() -> Result<VecDeque<String>, ArgumentError> {
    let mut args = env::args()
        .into_iter()
        .skip(1)
        .collect::<VecDeque<String>>();
    args.get(0).ok_or(NO_ARGUMENT_ERROR)?;
    Ok(args)
}

fn parse_args(mut args: VecDeque<String>) -> Result<GlobalSettings, ArgumentError> {
    let mut settings = GlobalSettings::new();

    while !args.is_empty() {
        let arg = args.pop_front().unwrap();

        // Call version and exit early if arg is version
        arg.eq("version").then(|| print_version());
        // Call help and exit early if arg is help
        arg.eq("help").then(|| print_help());

        is_arg_valid(&arg)?;

        // TODO: Come up with a better way to handle the help argument
        if arg.eq("-h") || arg.eq("--help") {
            let f = flags::get_help(&arg)?;
            println!("{}", f.help());
            std::process::exit(0x00);
        }

        if let Some(arg2) = args.front() {
            if arg2.eq("-h") || arg2.eq("--help") {
                let f = flags::get_help(&arg)?;
                println!("{}", f.help());
                std::process::exit(0x00);
            }
        };

        if let Some(f) = flags::get_input(&arg) {
            let input = args.pop_front().ok_or(MISSING_INPUT_ERROR)?;
            let setting = f.produce_input_setting(&input)?;
            settings.add_setting(setting);
        } else if let Some(f) = flags::get_toggle(&arg) {
            let setting = f.produce_toggle_setting();
            settings.add_setting(setting);
        } else {
            return Err(INVALID_ARGUMENT_ERROR);
        }
    }
    Ok(settings)
}

fn is_arg_valid(value: &str) -> Result<(), ArgumentError> {
    let option = Regex::new(r"^--?[aA-zZ]+$").unwrap();
    if let Some(v) = option.find(value) {
        return Ok(());
    }
    Err(INVALID_ARGUMENT_ERROR)
}
