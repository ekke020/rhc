pub(super) mod bruteforce;
pub(super) mod resource;
pub mod result;

struct Placeholder<'a> {
    target: &'a Vec<u8>,
    instant: std::time::Instant,
    algorithm: Box<dyn crate::algorithm::Algorithm>,
}
// TODO: Come up with a good entry point for this API
pub enum Mode {
    Resource,
    BruteForce,
}

pub type BruteForce = Mode;
pub type Dictionary = Mode;

fn calculate(
    bf: &mut BruteForce,
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
