pub mod charset;
pub mod dictionary;
pub mod error;
pub mod incremental;
pub mod spawn;
use self::error::core::CoreError;

pub type Error = error::core::CoreError;

// TODO: The dictionary and incremental modes share a lot of similarities
// TODO: Should introduce some polymorphism here.
// TODO: Should rethink this model so it wont be locked into a long loop and 
// TODO: can take updated instructions from the main thread.