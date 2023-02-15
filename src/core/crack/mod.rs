pub(super) mod dictionary;
pub(super) mod incremental;
pub mod mode;
pub mod result;
mod wrapper;

use self::wrapper::Wrapper;
use dictionary::Dictionary as DictionaryCore;
use incremental::Incremental as IncrementalCore;

pub type Incremental<'a> = Wrapper<IncrementalCore<'a>>;
pub type Dictionary<'a> = Wrapper<DictionaryCore<'a>>;

fn calculate(
    bf: &mut IncrementalCore,
    pm: Option<result::PasswordMatch>,
    length: usize,
    word: &mut Vec<u8>,
) -> Option<result::PasswordMatch> {
    if pm.is_some() {
        return pm;
    } else if length == 0 {
        return None;
    }
    super::constants::NO_SPECIAL_RANGE.iter().for_each(|c| {
        word.push(*c);
        calculate(bf, None, length - 1, word);
        word.pop();
    });
    None
}

pub fn execute_comparison(
    algorithm: &mut dyn crate::algorithm::Algorithm,
    word: &[u8],
    target: &Vec<u8>,
) -> bool {
    algorithm.populate(word);
    algorithm.execute();
    algorithm.compare(target)
}
