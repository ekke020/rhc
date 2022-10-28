use crate::sha2::bit_utils::u64_addition;

use super::bit_utils::{right_shift, u64_rotate};
use super::consts::{State512, H512_384, H512_512, K64};
use super::implementation::{CompressionSize, Hash, Sha, U48, U64};
pub struct Sha512 {
    value: Vec<u8>, // The value that was provided.
    state: State512,
    compressed: Option<U64>, // The final hash from the value.
}

impl Sha for Sha512 {
    fn new(value: Vec<u8>) -> Self {
        Self {
            value,
            state: H512_512,
            compressed: None,
        }
    }
}

impl Hash<U64> for Sha512 {
    fn reload(&mut self, value: Vec<u8>) {
        self.value = value;
    }

    fn run(&mut self) {
        let mut buffer = self.state;
        for chunk in self.value.chunks_mut(128) {
            let message = mutate_chunk(chunk);
            buffer = compression(message, buffer);
        }
        self.compressed = Some(U64::transform(buffer));
    }

    fn extract(&mut self) -> U64 {
        self.compressed
            .take()
            .expect("Can't extract before running hash")
    }
}

pub struct Sha384 {
    value: Vec<u8>, // The value that was provided.
    state: State512,
    compressed: Option<U48>, // The final hash from the value.
}

impl Sha for Sha384 {
    fn new(value: Vec<u8>) -> Self {
        Self {
            value,
            state: H512_384,
            compressed: None,
        }
    }
}

impl Hash<U48> for Sha384 {
    fn reload(&mut self, value: Vec<u8>) {
        self.value = value;
    }

    fn run(&mut self) {
        let mut buffer = self.state;
        for chunk in self.value.chunks_mut(128) {
            let message = mutate_chunk(chunk);
            buffer = compression(message, buffer);
        }
        self.compressed = Some(U48::transform(buffer));
    }

    fn extract(&mut self) -> U48 {
        self.compressed
            .take()
            .expect("Can't extract before running hash")
    }
}

fn mutate_chunk(message: &[u8]) -> [u64; 80] {
    let mut message_schedule: [u64; 80] = [0; 80];
    let mut i = 0;

    // Concatenate 8 bytes into a 64bit word.
    message.windows(8).step_by(8).for_each(|bytes| {
        message_schedule[i] = concatenate_bytes(bytes);
        i += 1;
    });

    // Manipulate the bits after index 15.
    for i in 16..80 {
        let s0: u64 = bit_manipulation(&message_schedule[i - 15], 1, 8, 7);
        let s1: u64 = bit_manipulation(&message_schedule[i - 2], 19, 61, 6);
        let [s2, s3] = [message_schedule[i - 16], message_schedule[i - 7]];
        message_schedule[i] = u64_addition!(s0, s1, s2, s3);
    }
    message_schedule
}

fn concatenate_bytes(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .map(|byte| *byte as u64)
        .reduce(|con, byte| (con << 8) | byte)
        .expect("Concatenation can't be performed on an empty array.")
}

fn bit_manipulation(word_64_bit: &u64, r1: u8, r2: u8, r3: u8) -> u64 {
    let n0 = u64_rotate(word_64_bit, r1);
    let n1 = u64_rotate(word_64_bit, r2);
    let n2 = right_shift!(word_64_bit, r3);
    n0 ^ n1 ^ n2
}

fn compression(message: [u64; 80], compressed: [u64; 8]) -> [u64; 8] {
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = compressed;

    for i in 0..80 {
        let s1 = u64_rotate(&e, 14) ^ u64_rotate(&e, 18) ^ u64_rotate(&e, 41);
        let ch = (e & f) ^ ((!e) & g);
        let temp1 = u64_addition!(h, s1, ch, K64[i], message[i]);
        let s0 = u64_rotate(&a, 28) ^ u64_rotate(&a, 34) ^ u64_rotate(&a, 39);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = u64_addition!(s0, maj);
        h = g;
        g = f;
        f = e;
        e = u64_addition!(d, temp1);
        d = c;
        c = b;
        b = a;
        a = u64_addition!(temp1, temp2);
    }
    let compressed: [u64; 8] = [
        u64_addition!(compressed[0], a),
        u64_addition!(compressed[1], b),
        u64_addition!(compressed[2], c),
        u64_addition!(compressed[3], d),
        u64_addition!(compressed[4], e),
        u64_addition!(compressed[5], f),
        u64_addition!(compressed[6], g),
        u64_addition!(compressed[7], h),
    ];
    compressed
}
