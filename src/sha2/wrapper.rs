use super::implementation::{Extract, Sha};
use crate::sha2::implementation::{CompressionSize, Hash};
use std::marker::PhantomData;
pub struct Wrapper<T, U> {
    sha2: T,
    compression: PhantomData<U>,
}
impl<T, U> Wrapper<T, U>
where
    T: Hash<U>,
    U: CompressionSize,
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
    U: CompressionSize + Extract,
{
    pub fn extract(&mut self) -> Vec<<U as Extract>::Size> {
        let value = self.sha2.extract();
        value.take()
    }

    pub fn extract_as_lower_hex(&mut self) -> String {
        let value = self.extract();
        value.iter().map(|dec| format!("{:x}", dec)).collect()
    }

    pub fn extract_as_upper_hex(&mut self) -> String {
        let value = self.extract();
        value.iter().map(|dec| format!("{:X}", dec)).collect()
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
type ByteSize = usize;
const BYTE_SIZE_64: ByteSize = 64;
const BYTE_SIZE_128: ByteSize = 128;

fn get_decimals(bytes: &[u8], size: ByteSize) -> Vec<u8> {
    let mut decimal = lazy_vector!(bytes.len(), size);

    // Add the binary values to the array.
    bytes
        .iter()
        .enumerate()
        .for_each(|(i, byte)| decimal[i] = *byte);

    // Append a single bit after the last binary.
    decimal[bytes.len()] = 0x80;

    // Get the big endian representation of the length of value.
    let big_endian_rep = (bytes.len() * 8).to_be_bytes();
    big_endian_rep
        .iter()
        .for_each(|byte| decimal.push(*byte));
    decimal
}

#[test]
fn test_get_decimals() {
    let test = "test";
    let k = get_decimals(test.as_bytes());
    assert_eq!([k[0], k[1], k[2], k[3], k[4]], [116, 101, 115, 116, 128]);
    assert_eq!(k[63], 32)
}
