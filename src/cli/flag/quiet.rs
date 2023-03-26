use super::*;

const SHORTHAND: char = 'q';
const NAME: &str = "quiet";
const SHORT_HELP: &str = "suppresses all non-essential output";
const LONG_HELP: &str = "Flag: -q | --quiet
Details:
    Toggles non essential output.
Description:
    RHC runs in verbose mode by default and this can be overridden with the --quiet flag.
    Toggling quiet mode will suppress all non-essential output, this includes any status
    updates of the crack as well as any statistical outputs. There is a slight performance
    improvement when running in quiet mode compared to the default verbose mode.
    Note that the final result of the crack wont be suppressed by running in quiet mode.
Example: 
    rhc [OPTIONS]... -q 
";
pub(super) struct Quiet;

impl FlagInfo for Quiet {
    fn describe(&self) -> String {
        format!("-{SHORTHAND}, --{NAME} \t\t{SHORT_HELP}")
    }
}

impl FlagHelp for Quiet {
    fn help(&self) -> String {
        LONG_HELP.to_owned()
    }
}

impl FlagToggle for Quiet {
    fn produce_toggle_setting(&self) -> Setting {
        Setting::Quiet(true)
    }
}
