pub mod charset;
pub mod dictionary;
pub mod error;
pub mod incremental;
pub mod result;
use self::error::core::CoreError;

pub type Error = error::core::CoreError;
