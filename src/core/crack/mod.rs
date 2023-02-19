pub mod consts;
mod dictionary;
mod incremental;
pub mod mode;
pub mod result;
mod wrapper;

use self::wrapper::Wrapper;
use dictionary::Dictionary as DictionaryCore;
use incremental::Incremental as IncrementalCore;

pub type Mode = mode::Mode;
pub type Incremental = Wrapper<IncrementalCore>;
pub type Dictionary<'a> = Wrapper<DictionaryCore<'a>>;


// #[cfg(test)]
// mod test {
//     use crate::algorithm::AlgorithmType;

//     #[test]
//     fn test_execute_comparison() {
//         let target = vec![
//             219, 60, 218, 134, 212, 66, 154, 29, 57, 193, 72, 152, 149, 102, 179, 143, 123, 218, 1,
//             86, 41, 107, 211, 100, 186, 47, 135, 139,
//         ];
//         let mut algorithm = AlgorithmType::from("sha2_224").unwrap().get_algorithm();
//         assert!(execute_comparison(
//             algorithm.as_mut(),
//             "ab".as_bytes(),
//             &target
//         ));
//     }
// }
