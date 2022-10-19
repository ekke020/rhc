use std::convert::TryInto;
// TODO: Think of a better name for this file.
pub struct Sha256;
pub struct Sha512;
pub trait CompressionSize<T: Sized, const N: usize> {
    fn transform(compressed: [T; N]) -> Self;

}
pub trait Extract<T: Sized, const N: usize> {
    fn take(self) -> [T; N];
}
pub struct U32([u32; 8]);
impl CompressionSize<u32, 8> for U32 {
    fn transform(compressed: [u32; 8]) -> Self {
        U32(compressed)
    }
}
impl Extract<u32, 8> for U32 {
    fn take(self) -> [u32; 8] {
        self.0
    }
}
pub struct U28([u32; 7]);
impl CompressionSize<u32, 8> for U28 {
    fn transform(compressed: [u32; 8]) -> Self {
        U28(compressed[0..7].try_into().unwrap())
    }
}
impl Extract<u32, 7> for U28 {
    fn take(self) -> [u32; 7] {
        self.0
    }
}

pub trait Hash<T: CompressionSize<u32, 8>> {
    fn reload(&mut self, value: Vec<u8>);

    fn run(&mut self);

    fn extract(&mut self) -> T;

    // fn compression(mutated: [u32; 64]) -> T;
}

pub trait Sha {
    fn new(value: Vec<u8>) -> Self;

}
