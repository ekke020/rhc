#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Mode {
    Dictionary,
    Incremental,
}
impl Mode {
    pub fn from(value: &str) -> Option<Self> {
        match value {
            "dictionary" => Some(Self::Dictionary),
            "incremental" => Some(Self::Incremental),
            _ => None,
        }
    }
}
// TODO: Redesign this. Probably move it as well?
enum ModeSettings<'a> {
    Dictionary(DictionarySettings<'a>),
    Incremental(IncrementalSettings),
}

struct IncrementalSettings {
    max_length: usize,
    min_length: usize,
    range: &'static [u8],
}

struct DictionarySettings<'a> {
    wordlist: &'a [String],
}
// TODO: This structure should be passed to each thread.
// TODO: Create this structure from the cli settings. Generate an array of this structure where each modeSettings contains the relevant info for target thread.
struct Test<'a> {
    target: &'a Vec<u8>,
    thread_count: &'a usize,
    algorithm: &'a crate::algorithm::AlgorithmType,
    verbose: &'a bool,
    modes: [Option<ModeSettings<'a>>; 2],
    // modes: HashSet<Mode>,
    // wordlist: Option<Vec<String>>,
    // min_length: usize,
    // max_length: usize,
}