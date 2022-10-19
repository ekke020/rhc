use super::{
    sha256_core::Sha256,
    implementation::{Extract, Sha},
};
use crate::sha2::implementation::{CompressionSize, Hash, U32};
use std::marker::PhantomData;
pub struct Wrapper<T, U> {
    sha2: T,
    compression: PhantomData<U>,
}
impl<T, U> Wrapper<T, U>
where
    T: Hash<U>,
    U: CompressionSize<u32, 8>,
{
    pub fn run(&mut self) {
        self.sha2.run();
    }

    pub fn reload(&mut self, data: impl AsRef<[u8]>) {
        self.sha2.reload(get_decimals(data.as_ref()))
    }
}

impl<T, U> Wrapper<T, U>
where
    T: Hash<U>,
    U: CompressionSize<u32, 8> + Extract<u32, 8>,
{
    // TODO: Figure out how to make this more generic
    // TODO: Wrapper has an implementation for this and it might be bad...
    pub fn extract(&mut self) -> [u32; 8] {
        let value = self.sha2.extract();
        value.take()
    }
}

impl<T, U> Wrapper<T, U>
where
    T: Sha,
{
    pub fn new(data: impl AsRef<[u8]>) -> Self {
        Self {
            sha2: T::new(get_decimals(data.as_ref())),
            compression: PhantomData,
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

#[test]
fn test_get_decimals() {
    let test = "test";
    let k = get_decimals(test.as_bytes());
    assert_eq!([k[0], k[1], k[2], k[3], k[4]], [116, 101, 115, 116, 128]);
    assert_eq!(k[63], 32)
}