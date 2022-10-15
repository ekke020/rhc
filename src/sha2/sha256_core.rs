// round constants for sha256
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

// Sha256 constants representing the first 32 bits of the square root of the first 8 primes.
const H0: u32 = 0x6a09e667;
const H1: u32 = 0xbb67ae85;
const H2: u32 = 0x3c6ef372;
const H3: u32 = 0xa54ff53a;
const H4: u32 = 0x510e527f;
const H5: u32 = 0x9b05688c;
const H6: u32 = 0x1f83d9ab;
const H7: u32 = 0x5be0cd19;

const ADDITION_OVERFLOW: u64 = 4294967296;

pub struct Sha256<'a> {
    value: &'a [u8], // The value that was provided.
    state: State256,
    compressed: [u32; 8], // The final hash from the value.
}

impl<'a> Sha256<'a> {
    pub fn new(value: &'a [u8]) -> Self {
        Self {
            value,
            state: H256_256,
            compressed: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
        }
    }
    pub fn run(&mut self) -> Option<[u32; 8]> {
        let mut decimal = get_decimals(self.value);
        for chunk in decimal.chunks_mut(64) {
            let word_32_bit = mutate_chunk(chunk);
            self.compressed = compression(word_32_bit, self.compressed);
        }
        Some(self.compressed)
        // TODO: Dont compare in here, just send the compressed value back
    }
}

pub struct Sha224<'a> {
    value: &'a [u8], // The value that was provided.
    state: State256,
    compressed: Option<U28>,
}
impl<'a> Sha224<'a> {
    pub fn new(value: &'a [u8]) -> Self {
        Self {
            value,
            state: H256_224,
            compressed: None,
        }
    }
    fn compression(&self, mutated: [u32; 64]) -> U28 {
        // TODO: Move this out of impl block
        let mut v = self.state;

        for i in 0..64 {
            let s1 = right_rotate(&v[4], 6) ^ right_rotate(&v[4], 11) ^ right_rotate(&v[4], 25);
            let ch = (v[4] & v[5]) ^ ((!v[4]) & v[6]);
            let temp1 = addition_with_overflow(&[v[7], s1, ch, K[i], mutated[i]]);
            let s0 = right_rotate(&v[0], 2) ^ right_rotate(&v[0], 13) ^ right_rotate(&v[0], 22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let temp2 = addition_with_overflow(&[s0, maj]);
            v[7] = v[6];
            v[6] = v[5];
            v[5] = v[4];
            v[4] = addition_with_overflow(&[v[3], temp1]);
            v[3] = v[2];
            v[2] = v[1];
            v[1] = v[0];
            v[0] = addition_with_overflow(&[temp1, temp2]);
        }
        let compressed: [u32; 8] = [
            addition_with_overflow(&[self.state[0], v[0]]),
            addition_with_overflow(&[self.state[1], v[1]]),
            addition_with_overflow(&[self.state[2], v[2]]),
            addition_with_overflow(&[self.state[3], v[3]]),
            addition_with_overflow(&[self.state[4], v[4]]),
            addition_with_overflow(&[self.state[5], v[5]]),
            addition_with_overflow(&[self.state[6], v[6]]),
            addition_with_overflow(&[self.state[7], v[7]]),
        ];
        U28::transform(compressed)
    }
}
use super::{
    consts::{State256, H256_224, H256_256},
    wrapper::{CompressionSize, U28}, bit_utils::u32_addition,
};
use crate::sha2::{wrapper::Hash, bit_utils::lazy_vector};
impl<'a> Hash<U28> for Sha224<'a> {
    fn reload() {
        todo!()
    }

    fn run(&mut self) {
        let mut decimal = get_decimals(self.value);
        for chunk in decimal.chunks_mut(64) {
            let word_32_bit = mutate_chunk(chunk);
            self.compressed = Some(self.compression(word_32_bit));
        }
    }

    fn extract(&mut self) -> U28 {
        let content = self
            .compressed
            .take()
            .expect("Can't extract before running hash");
        content
    }
}

fn find_multiple(len: usize) -> usize {
    let m = (len) / (56);
    let cap = (m + 1) * 56;
    println!("{}", cap);
    cap
}

fn get_decimals(bytes: &[u8]) -> Vec<u8> {
    find_multiple(bytes.len());
    let mut decimal_256 = lazy_vector!(bytes.len(), 64);

    // Add the binary values to the array.
    bytes
        .iter()
        .enumerate()
        .for_each(|(i, byte)| decimal_256[i] = *byte);

    // Append a single bit after the last binary.
    decimal_256[bytes.len()] = 0x80;

    // Get the big endian representation of the length of value.
    println!("before adding big endian rep: {:x?}", decimal_256);
    println!("Length: {}", decimal_256.len());
    let big_endian_rep = (bytes.len() * 8).to_be_bytes();
    big_endian_rep
        .iter()
        .for_each(|byte| decimal_256.push(*byte));

    println!("When done: {:x?}", decimal_256);
    println!("Length: {}", decimal_256.len());
    decimal_256
}

fn mutate_chunk(decimals: &[u8]) -> [u32; 64] {
    let mut word_32_bit: [u32; 64] = [0; 64];
    let mut i = 0;

    // Concatenate 4 bytes into a 32bit word.
    decimals.windows(4).step_by(4).for_each(|bytes| {
        word_32_bit[i] = concatenate_bytes(bytes);
        i += 1;
    });

    // Manipulate the bits after index 15.
    for i in 16..64 {
        let s0: u32 = bit_manipulation(&word_32_bit[i - 15], 7, 18, 3);
        let s1: u32 = bit_manipulation(&word_32_bit[i - 2], 17, 19, 10);
        let [s2, s3] = [word_32_bit[i - 16], word_32_bit[i - 7]];
        word_32_bit[i] = u32_addition!(s0, s1, s2, s3);
    }
    word_32_bit
}

fn compression(mutated: [u32; 64], test: [u32; 8]) -> [u32; 8] {
    let mut a: u32 = test[0];
    let mut b: u32 = test[1];
    let mut c: u32 = test[2];
    let mut d: u32 = test[3];
    let mut e: u32 = test[4];
    let mut f: u32 = test[5];
    let mut g: u32 = test[6];
    let mut h: u32 = test[7];

    for i in 0..64 {
        let s1 = right_rotate(&e, 6) ^ right_rotate(&e, 11) ^ right_rotate(&e, 25);
        let ch = (e & f) ^ ((!e) & g);
        let temp1 = u32_addition!(h, s1, ch, K[i], mutated[i]);
        let s0 = right_rotate(&a, 2) ^ right_rotate(&a, 13) ^ right_rotate(&a, 22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = u32_addition!(s0, maj);
        h = g;
        g = f;
        f = e;
        e = u32_addition!(d, temp1);
        d = c;
        c = b;
        b = a;
        a = u32_addition!(temp1, temp2);
    }
    let compressed: [u32; 8] = [
        u32_addition!(test[0], a),
        u32_addition!(test[1], b),
        u32_addition!(test[2], c),
        u32_addition!(test[3], d),
        u32_addition!(test[4], e),
        u32_addition!(test[5], f),
        u32_addition!(test[6], g),
        u32_addition!(test[7], h),
    ];
    compressed
}

fn addition_with_overflow(args: &[u32]) -> u32 {
    args.iter()
        .map(|arg| *arg as u64)
        .reduce(|total, arg| (total + arg) % ADDITION_OVERFLOW)
        .expect("Empty array is not allowed") as u32
}

fn bit_manipulation(word_32_bit: &u32, r1: u8, r2: u8, r3: u8) -> u32 {
    let n0 = right_rotate(word_32_bit, r1);
    let n1 = right_rotate(word_32_bit, r2);
    let n2 = right_shift(word_32_bit, r3);
    n0 ^ n1 ^ n2
}

fn concatenate_bytes(bytes: &[u8]) -> u32 {
    let mut decimal_rep: u32 = bytes[0].into();
    decimal_rep = (decimal_rep << 8) | bytes[1] as u32;
    decimal_rep = (decimal_rep << 8) | bytes[2] as u32;
    decimal_rep = (decimal_rep << 8) | bytes[3] as u32;
    decimal_rep
}

fn right_rotate(n: &u32, d: u8) -> u32 {
    (n >> d) | (n << (32 - d))
}

fn right_shift(n: &u32, d: u8) -> u32 {
    n >> d
}

mod test {
    use crate::sha2::sha256_core::*;

    #[test]
    fn test_right_rotate() {
        assert_eq!(right_rotate(&0x9B05688C, 5), 0x64D82B44);
        assert_eq!(right_rotate(&0x9B05688C, 10), 0x2326C15A);
        assert_ne!(right_rotate(&0x9B05688C, 2), 0x4464D82B);
    }

    #[test]
    fn test_right_shift() {
        assert_eq!(right_shift(&0x9B05688C, 7), 0x1360AD1);
        assert_eq!(right_shift(&0x9B05688C, 10), 0x26C15A);
        assert_ne!(right_shift(&0x9B05688C, 2), 0x9B05688);
    }
    #[test]
    fn test_concatenate_bytes() {
        let bytes = "test".as_bytes();
        assert_eq!(concatenate_bytes(bytes), 0x74657374);
        assert_ne!(concatenate_bytes(bytes), 0x74657375);
    }

    #[test]
    fn test_bit_manipulation() {
        assert_eq!(bit_manipulation(&0x74657374, 5, 10, 5), 0x7D1D195C);
        assert_eq!(bit_manipulation(&0x74657374, 7, 18, 3), 0xBAB97991);
        assert_ne!(bit_manipulation(&0x74657374, 7, 18, 3), 0xBAB97990);
    }

    #[test]
    fn test_addition_with_overflow() {
        assert_eq!(
            addition_with_overflow(&[0x74657374, 0x676F6F64]),
            0xDBD4E2D8
        );
        assert_ne!(
            addition_with_overflow(&[0x7361640A, 0x6C75636B]),
            0xDBD4E2D8
        );
    }

    #[test]
    fn test_get_decimals() {
        let test = "test";
        let k = get_decimals(test.as_bytes());
        assert_eq!([k[0], k[1], k[2], k[3], k[4]], [116, 101, 115, 116, 128]);
        assert_eq!(k[63], 32)
    }

    #[test]
    fn test_mutate_chunk() {
        // Decimals of "test"
        let decimals: Vec<u8> = vec![
            0x74, 0x65, 0x73, 0x74, 0x80, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20,
        ];
        let correct: [u32; 64] = [
            0x74657374, 0x80000000, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x20, 0x85659374, 0x80140000, 0x7bf58b7a, 0x80205508, 0x74cc8fe6, 0x20055801,
            0xd612c7fc, 0x8c6e48c8, 0xbb48757a, 0x6953d7a2, 0xb45d2dd8, 0x60bbd5c, 0x537fb3ef,
            0x7f16c927, 0xfc14e508, 0x166c6386, 0xedd657cc, 0x8b7f453f, 0x776c519d, 0xff4489c8,
            0xe705110d, 0x448e3765, 0x29c4f03b, 0x56d4fa86, 0xe8e882ae, 0xaf5bb0c4, 0x5c74ac3c,
            0xd394c0d8, 0x4ef1cf66, 0xd857da58, 0x4737038f, 0x2738a62e, 0xbe10843f, 0x50331a18,
            0x4a1ce75b, 0x7fff59c9, 0xfe72c27a, 0x22ed8860, 0xc321f5c0, 0xea81a878, 0x6e0938fe,
            0x32bbcc5b, 0x33d3040f, 0x284c1f19, 0xb0964602, 0xfe6ad1fb, 0x8ec8c416, 0x11f0d783,
        ];
        let mutated = mutate_chunk(&decimals);
        assert_eq!(mutated, correct);
    }

    #[test]
    fn test_compression() {
        // mutated string of "test"
        let chunk: [u32; 64] = [
            0x74657374, 0x80000000, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x20, 0x85659374, 0x80140000, 0x7bf58b7a, 0x80205508, 0x74cc8fe6, 0x20055801,
            0xd612c7fc, 0x8c6e48c8, 0xbb48757a, 0x6953d7a2, 0xb45d2dd8, 0x60bbd5c, 0x537fb3ef,
            0x7f16c927, 0xfc14e508, 0x166c6386, 0xedd657cc, 0x8b7f453f, 0x776c519d, 0xff4489c8,
            0xe705110d, 0x448e3765, 0x29c4f03b, 0x56d4fa86, 0xe8e882ae, 0xaf5bb0c4, 0x5c74ac3c,
            0xd394c0d8, 0x4ef1cf66, 0xd857da58, 0x4737038f, 0x2738a62e, 0xbe10843f, 0x50331a18,
            0x4a1ce75b, 0x7fff59c9, 0xfe72c27a, 0x22ed8860, 0xc321f5c0, 0xea81a878, 0x6e0938fe,
            0x32bbcc5b, 0x33d3040f, 0x284c1f19, 0xb0964602, 0xfe6ad1fb, 0x8ec8c416, 0x11f0d783,
        ];
        // correct result of compression
        let correct: [u32; 8] = [
            0x9f86d081, 0x884c7d65, 0x9a2feaa0, 0xc55ad015, 0xa3bf4f1b, 0x2b0b822c, 0xd15d6c15,
            0xb0f00a08,
        ];
        // assert_eq!(compression(chunk), correct);
    }
}
