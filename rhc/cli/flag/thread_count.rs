use super::*;

const NAME: &str = "thread-count";
const SHORT_HELP: &str = "Specifies the number of threads to utilize";
const LONG_HELP: &str = "Flag: --thread-count
Details:
    input type: unsigned 32 bit integer
Description:
    The thread-count flag can be used to specify the number of threads to
    utilize when running a crack. Its important to note that trying to target
    a thread count that is higher than what is available will result in a termination.
    If the goal is to use all available threads then this flag can be ignored.
Example: 
    rhc [OPTIONS]... --thread-count 4
";

pub(super) struct ThreadCount;

impl FlagInfo for ThreadCount {
    fn describe(&self) -> String {
        format!("    --{NAME} \t\t{SHORT_HELP}")
    }
}

impl FlagHelp for ThreadCount {
    fn help(&self) -> String {
        LONG_HELP.to_owned()
    }
}

impl FlagInput for ThreadCount {
    fn produce_input_setting(&self, value: &str) -> Result<Setting, ArgumentError> {
        let count = value.parse::<usize>().ok().ok_or(INVALID_INPUT_ERROR)?;
        Ok(Setting::ThreadCount(count))
    }
}
