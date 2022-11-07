use std::{convert::TryInto, fmt::{LowerHex, UpperHex}};
use super::consts::{ByteSize, BYTE_SIZE_64, BYTE_SIZE_128};

pub trait CompressionSize {
    type Size: Sized;
    fn transform(compressed: [Self::Size; 8]) -> Self;
}
pub trait Extract {
    const S: usize;
    fn take<const N: usize>(self) -> [u8; N];
}
pub struct U64([u8; 64]);
impl CompressionSize for U64 {
    type Size = u64;
    fn transform(compressed: [u64; 8]) -> Self {
        let mut test: [u8; 64] = [0; 64];
        let mut i = 0;
        for value in compressed {
            for byte in value.to_be_bytes() {
                test[i] = byte;
                i += 1;
            }
        }
    //    let test: [u8; 64] = compressed.iter()
    //         .map(|s| s.as_bytes())
    //         .flat_map(|bytes| bytes)
    //         .map(|f| *f)
    //         .try_into()
    //         .unwrap();
            // .try_into()
            // .unwrap();
            // .try_into()
            // .unwrap();
        U64(test)
    }
}
impl Extract for U64 {
    const S: usize = 64;
    fn take<const N: usize>(self) -> [u8; N] {
        self.0[0..N].try_into().unwrap()
    }
}

pub trait Hash<T>
where T: CompressionSize
{
    fn reload(&mut self, value: &[u8]);

    fn run(&mut self);

    fn extract(&mut self) -> T;

}
pub trait Sha {
    fn new(value: &[u8]) -> Self;
}

use std::marker::PhantomData;

pub struct Test<T, U, const N: usize> {
    sha2: T,
    compression: PhantomData<U>,
}

impl<T, U, const N: usize> Test<T, U, N>
where
    T: Hash<U>,
    U: CompressionSize,
{
    pub fn run(&mut self) {
        self.sha2.run();
    }

    pub fn reload(&mut self, data: impl AsRef<[u8]>) {
        self.sha2.reload(data.as_ref());
    }
}

impl<T, U, const N: usize> Test<T, U, N>
where
    T: Hash<U>,
    U: CompressionSize + Extract,
{
    pub fn extract(&mut self) -> [u8; N] {
        let value = self.sha2.extract();
        value.take::<N>()
    }

    pub fn extract_as_lower_hex(&mut self) -> String {
        self.extract()
            .iter()
            .map(|c| c.to_be_bytes())
            .flat_map(|byte| byte)
            .map(|f| format!("{:01$x}", f, 2))
            .collect()
        // value.iter().map(|dec| format!("{:01$x?}", dec, 16)).collect()
    }

    pub fn extract_as_upper_hex(&mut self) -> String {
        self.extract().iter()
            .map(|dec| format!("{:X}", dec))
            .collect()
    }
}

impl<T, U, const N: usize> Test<T, U, N>
where
    T: Sha,
    U: Extract,
{
    pub fn new(data: impl AsRef<[u8]>) -> Self {
        Self {
            sha2: T::new(data.as_ref()),
            compression: PhantomData,
        }
    }
}