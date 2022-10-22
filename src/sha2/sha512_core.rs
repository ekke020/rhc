use super::consts::{State512, H512_512, H512_384, K64};
use super::implementation::U64;
pub struct Sha512 {
    value: Vec<u8>, // The value that was provided.
    state: State512,
    compressed: Option<U64>, // The final hash from the value.
}

pub struct Sha384 {
    value: Vec<u8>, // The value that was provided.
    state: State512,
    compressed: Option<U64>, // The final hash from the value.
}