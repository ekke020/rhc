use std::ops::Shr;

use super::super::common::{pad, u32_addition, to_bytes_32, RUN_ERROR, EXTRACT_ERROR};
use super::consts::{State256, H256_224, H256_256, K32};
use super::super::compression::{CompressionSize, Hash, Sha, U28, U32};

pub struct Sha256 {
    value: Option<Vec<u8>>, // The value that was provided.
    state: State256,
    compressed: Option<U32>, // The final hash from the value.
}

impl Sha for Sha256 {
    fn from(value: &[u8]) -> Self {
        Self {
            value: Some(pad::<64>(value)),
            ..Default::default()
        }
    }

    fn new() -> Self {
        Self::default()
    }
}

impl Default for Sha256 {
    fn default() -> Self {
        Self { value: None, state: H256_256, compressed: None }
    }
}

impl Hash<U32> for Sha256 {
    fn load(&mut self, value: &[u8]) {
        self.value = Some(pad::<64>(value));
    }

    fn run(&mut self) {
        let mut buffer = self.state;
        let mut value = self.value.take().expect(RUN_ERROR);
        for block in value.chunks_mut(64) {
            let message = prepare_block(block);
            buffer = compression(message, buffer);
        }
        let bytes = to_bytes_32::<32>(&buffer);
        self.compressed = Some(U32::new(&bytes));
    }

    fn extract(&mut self) -> U32 {
        self.compressed
            .take()
            .expect(EXTRACT_ERROR)
    }
}

pub struct Sha224 {
    value: Option<Vec<u8>>, // The value that was provided.
    state: State256,
    compressed: Option<U28>,
}

impl Sha for Sha224 {
    fn from(value: &[u8]) -> Self {
        Self {
            value: Some(pad::<64>(value)),
            ..Default::default()
        }
    }

    fn new() -> Self {
        Self::default()
    }
}

impl Default for Sha224 {
    fn default() -> Self {
        Self { value: None, state: H256_224, compressed: None }
    }
}

impl Hash<U28> for Sha224 {
    fn load(&mut self, value: &[u8]) {
        self.value = Some(pad::<64>(value));
    }

    fn run(&mut self) {
        let mut buffer = self.state;
        let mut value = self.value.take().expect(RUN_ERROR);
        for block in value.chunks_mut(64) {
            let message = prepare_block(block);
            buffer = compression(message, buffer);
        }
        let bytes = to_bytes_32::<28>(&buffer);
        self.compressed = Some(U28::new(&bytes));
    }

    fn extract(&mut self) -> U28 {
        self.compressed
            .take()
            .expect(EXTRACT_ERROR)
    }
}

fn prepare_block(message: &[u8]) -> [u32; 64] {
    let mut message_schedule: [u32; 64] = [0; 64];
    
    // Concatenate 4 bytes into a 32bit word.
    message.windows(4).step_by(4).enumerate().for_each(|(i, bytes)| {
        message_schedule[i] = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    });

    // Manipulate the bits after index 15.
    for i in 16..64 {
        let s0: u32 = bit_manipulation(&message_schedule[i - 15], 7, 18, 3);
        let s1: u32 = bit_manipulation(&message_schedule[i - 2], 17, 19, 10);
        let [s2, s3] = [message_schedule[i - 16], message_schedule[i - 7]];
        message_schedule[i] = u32_addition!(s0, s1, s2, s3);
    }
    message_schedule
}

fn bit_manipulation(word_32_bit: &u32, r1: u32, r2: u32, r3: u32) -> u32 {
    let n0 = word_32_bit.rotate_right(r1);
    let n1 = word_32_bit.rotate_right(r2);
    let n2 = word_32_bit.shr(r3);
    n0 ^ n1 ^ n2
}

fn compression(message: [u32; 64], compressed: [u32; 8]) -> [u32; 8] {
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = compressed;

    for i in 0..64 {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ ((!e) & g);
        let temp1 = u32_addition!(h, s1, ch, K32[i], message[i]);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
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
        u32_addition!(compressed[0], a),
        u32_addition!(compressed[1], b),
        u32_addition!(compressed[2], c),
        u32_addition!(compressed[3], d),
        u32_addition!(compressed[4], e),
        u32_addition!(compressed[5], f),
        u32_addition!(compressed[6], g),
        u32_addition!(compressed[7], h),
    ];
    compressed
}

#[cfg(test)]
mod test {
    use super::*;


#[test]
fn bit_manipulation_returns_correct_values() {
    assert_eq!(bit_manipulation(&0x74657374, 5, 10, 5), 0x7D1D195C);
    assert_eq!(bit_manipulation(&0x74657374, 7, 18, 3), 0xBAB97991);
    assert_ne!(bit_manipulation(&0x74657374, 7, 18, 3), 0xBAB97990);
}

#[test]
fn prepare_block_returns_correct_values() {
    // pad of "test"
    let padded_value: Vec<u8> = vec![
        0x74, 0x65, 0x73, 0x74, 0x80, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20,
    ];
    let correct: [u32; 64] = [
        0x74657374, 0x80000000, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x20, 0x85659374, 0x80140000, 0x7bf58b7a, 0x80205508, 0x74cc8fe6, 0x20055801, 0xd612c7fc,
        0x8c6e48c8, 0xbb48757a, 0x6953d7a2, 0xb45d2dd8, 0x60bbd5c, 0x537fb3ef, 0x7f16c927,
        0xfc14e508, 0x166c6386, 0xedd657cc, 0x8b7f453f, 0x776c519d, 0xff4489c8, 0xe705110d,
        0x448e3765, 0x29c4f03b, 0x56d4fa86, 0xe8e882ae, 0xaf5bb0c4, 0x5c74ac3c, 0xd394c0d8,
        0x4ef1cf66, 0xd857da58, 0x4737038f, 0x2738a62e, 0xbe10843f, 0x50331a18, 0x4a1ce75b,
        0x7fff59c9, 0xfe72c27a, 0x22ed8860, 0xc321f5c0, 0xea81a878, 0x6e0938fe, 0x32bbcc5b,
        0x33d3040f, 0x284c1f19, 0xb0964602, 0xfe6ad1fb, 0x8ec8c416, 0x11f0d783,
    ];
    let mutated = prepare_block(&padded_value);
    assert_eq!(mutated, correct);
}

#[test]
fn compression_compresses() {
    // mutated string of "test"
    let chunk: [u32; 64] = [
        0x74657374, 0x80000000, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x20, 0x85659374, 0x80140000, 0x7bf58b7a, 0x80205508, 0x74cc8fe6, 0x20055801, 0xd612c7fc,
        0x8c6e48c8, 0xbb48757a, 0x6953d7a2, 0xb45d2dd8, 0x60bbd5c, 0x537fb3ef, 0x7f16c927,
        0xfc14e508, 0x166c6386, 0xedd657cc, 0x8b7f453f, 0x776c519d, 0xff4489c8, 0xe705110d,
        0x448e3765, 0x29c4f03b, 0x56d4fa86, 0xe8e882ae, 0xaf5bb0c4, 0x5c74ac3c, 0xd394c0d8,
        0x4ef1cf66, 0xd857da58, 0x4737038f, 0x2738a62e, 0xbe10843f, 0x50331a18, 0x4a1ce75b,
        0x7fff59c9, 0xfe72c27a, 0x22ed8860, 0xc321f5c0, 0xea81a878, 0x6e0938fe, 0x32bbcc5b,
        0x33d3040f, 0x284c1f19, 0xb0964602, 0xfe6ad1fb, 0x8ec8c416, 0x11f0d783,
    ];
    // correct result of compression
    let correct: [u32; 8] = [
        0x9f86d081, 0x884c7d65, 0x9a2feaa0, 0xc55ad015, 0xa3bf4f1b, 0x2b0b822c, 0xd15d6c15,
        0xb0f00a08,
    ];
    assert_eq!(compression(chunk, H256_256), correct);
}

}
