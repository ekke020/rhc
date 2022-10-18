use crate::sha2::wrapper::{Hash, CompressionSize, U32};
use std::marker::PhantomData;
use super::{sha256_core::Sha256, wrapper::Sha};
pub struct ShaWrapper<T, U>
{
    sha2: T,
    compression: PhantomData<U>
}
// TODO: Implement reload & extract from core.
// TODO: Think of a better name for the wrapper.
impl<T, U> ShaWrapper<T, U>
where T: Hash<U>,
U: CompressionSize<u32, 8>
{
    pub fn run(&mut self) {
        self.sha2.run();
    }
}

impl<'a, T, U> ShaWrapper<T, U>
where T: Sha
{
    pub fn new(value: impl AsRef<[u8]>) -> Self {
        Self {
            sha2: T::new(get_decimals(value.as_ref())),
            compression: PhantomData
        }
    }
}

use crate::sha2::bit_utils::lazy_vector;
fn get_decimals(bytes: &[u8]) -> Vec<u8> {
    let mut decimal_256 = lazy_vector!(bytes.len(), 64);

    // Add the binary values to the array.
    bytes
        .iter()
        .enumerate()
        .for_each(|(i, byte)| decimal_256[i] = *byte);

    // Append a single bit after the last binary.
    decimal_256[bytes.len()] = 0x80;

    // Get the big endian representation of the length of value.
    let big_endian_rep = (bytes.len() * 8).to_be_bytes();
    big_endian_rep
        .iter()
        .for_each(|byte| decimal_256.push(*byte));
    println!("Decimal rep of the bytes: {:?}", decimal_256);
    decimal_256
}