use std::{convert::TryInto, fmt::{LowerHex, UpperHex}};
use super::consts::ByteSize;

pub trait CompressionSize {
    type Size: Sized;
    fn transform(compressed: [Self::Size; 8]) -> Self;
}
pub trait Extract {
    type Size: Sized + LowerHex + UpperHex;
    fn take(self) -> Vec<Self::Size>;
}
pub struct U64([u64; 8]);
impl CompressionSize for U64 {
    type Size = u64;
    fn transform(compressed: [u64; 8]) -> Self {
        U64(compressed)
    }
}
impl Extract for U64 {
    type Size = u64;
    fn take(self) -> Vec<u64> {
        self.0.into() 
    }
}
pub struct U48([u64; 6]);
impl CompressionSize for U48 {
    type Size = u64;
    fn transform(compressed: [u64; 8]) -> Self {
        U48(compressed[0..6].try_into().unwrap())
    }
}
impl Extract for U48 {
    type Size = u64;
    fn take(self) -> Vec<u64> {
        self.0.into() 
    }
}
pub struct U32([u32; 8]);
impl CompressionSize for U32 {
    type Size = u32;
    fn transform(compressed: [u32; 8]) -> Self {
        U32(compressed)
    }
}
impl Extract for U32 {
    type Size = u32;
    fn take(self) -> Vec<u32> {
        self.0.into() 
    }
}
pub struct U28([u32; 7]);
impl CompressionSize for U28 {
    type Size = u32;
    fn transform(compressed: [u32; 8]) -> Self {
        U28(compressed[0..7].try_into().unwrap())
    }
}
impl Extract for U28 {
    type Size = u32; 
    fn take(self) -> Vec<u32> {
        self.0.into()
    }
}
pub trait Hash<T>
where T: CompressionSize
{
    fn reload(&mut self, value: Vec<u8>);

    fn run(&mut self);

    fn extract(&mut self) -> T;

}
pub trait Sha {
    fn new(value: Vec<u8>) -> Self;

}
