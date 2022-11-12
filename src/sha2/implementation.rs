use std::{convert::TryInto, fmt::{LowerHex, UpperHex}};

pub trait CompressionSize {
    fn new(compressed: &[u8]) -> Self;
}

pub trait Extract<const N: usize> {
    const S: usize;
    fn take(self) -> [u8; N];
}
pub struct U64([u8; 64]);
impl CompressionSize for U64 {
    fn new(compressed: &[u8]) -> Self {
        let data = compressed
            .try_into().expect("N has to be 64");
        U64(data)
    }
}
impl Extract<64> for U64 {
    const S: usize = 64;
    fn take(self) -> [u8; 64] {
        self.0
    }
}
pub struct U48([u8; 48]);
impl CompressionSize for U48 {
    fn new(compressed: &[u8]) -> Self {
        let data = compressed
            .try_into().expect("N has to be 48");
        U48(data)
    }
}
impl Extract<48> for U48 {
    const S: usize = 48;
    fn take(self) -> [u8; 48] {
        self.0
    }
}
pub struct U32([u8; 32]);
impl CompressionSize for U32 {
    fn new(compressed: &[u8]) -> Self {
        let data = compressed
            .try_into().expect("N has to be 32");
        U32(data)
    }
}
impl Extract<32> for U32 {
    const S: usize = 32;
    fn take(self) -> [u8; 32] {
        self.0
    }
}
pub struct U28([u8; 28]);
impl CompressionSize for U28 {
    fn new(compressed: &[u8]) -> Self {
        let data = compressed
            .try_into().expect("N has to be 28");
        U28(data)
    }
}
impl Extract<28> for U28 {
    const S: usize = 28;
    fn take(self) -> [u8; 28] {
        self.0
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
