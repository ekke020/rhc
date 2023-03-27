use std::fmt::Display;

use super::consts::StateM1;

#[macro_use]
macro_rules! lazy_vector {
    ($length:expr, $size:expr) => {{
        let m: usize = ($length + 8) / $size;
        let capacity: usize = ((m + 1) * ($size)) - 8;
        let mut temp: Vec<u8> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            temp.push(0x00);
        }
        temp
    }};
}
#[macro_use]
macro_rules! u32_addition {
    ($x:expr, $y:expr) => {
        (($x as u64 + $y as u64) % (u32::MAX as u64 + 1)) as u32
    };
    ($x:expr $( , $y:expr)*) => {
        (($x as u64 + u32_addition!($($y),*) as u64) % (u32::MAX as u64 + 1)) as u32
    };

}
#[macro_use]
macro_rules! left_shift {
    ($n:expr, $d:expr) => {
        $n << $d
    };
}

pub struct Sha {
    value: Option<Vec<u8>>, // The value that was provided.
    state: [u32; 5],
    // compressed: Option<U32>, // The final hash from the value.
}

impl Sha {
    fn from(value: &[u8]) -> Self {
        Self {
            value: Some(pad::<64>(value)),
            state: [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0],
        }
    }

    fn run(&mut self) -> String {
        let mut buffer = self.state;
        let mut value = self.value.take().unwrap();
        println!("Value : {}", value.len());
        for chunk in value.chunks_mut(64) {
            let message = mutate_chunk(chunk);
            buffer = compression(message, buffer);
        }
        let t = buffer[0].wrapping_shl(128)
            | buffer[1].wrapping_shl(96)
            | buffer[2].wrapping_shl(64)
            | buffer[3].wrapping_shl(32)
            | buffer[4];
        println!("{t}");
        let bytes = to_bytes::<20>(&buffer);
        bytes
            .iter()
            .map(|byte| format!("{:01$x}", byte, 2))
            .collect()
    }
}
fn mutate_chunk(message: &[u8]) -> [u32; 80] {
    let mut message_schedule: [u32; 80] = [0; 80];
    let mut i = 0;

    // Concatenate 4 bytes into a 32bit word.
    message.windows(4).step_by(4).for_each(|bytes| {
        message_schedule[i] = concatenate_bytes(bytes);
        i += 1;
    });

    // Manipulate the bits after index 15.
    for i in 16..79 {
        let w = message_schedule[i - 3]
            ^ message_schedule[i - 8]
            ^ message_schedule[i - 14]
            ^ message_schedule[i - 16];
        message_schedule[i] = left_rotate(&w, 1)
    }
    message_schedule
}

fn compression(message: [u32; 80], compressed: [u32; 5]) -> [u32; 5] {
    let [mut a, mut b, mut c, mut d, mut e] = compressed;

    for i in 0..79 {
        let [mut f, mut k] = [0, 0];
        if i <= 19 {
            f = (b & c) ^ ((!b) & d);
            k = 0x5A827999;
        } else if i <= 39 {
            f = b ^ c ^ d;
            k = 0x6ED9EBA1;
        } else if i <= 59 {
            f = (b & c) ^ (b & d) ^ (c & d);
            k = 0x8F1BBCDC;
        } else {
            f = b ^ c ^ d;
            k = 0xCA62C1D6;
        }
        let temp = u32_addition!(left_rotate(&a, 5), f, e, k, message[i]);
        e = d;
        d = c;
        c = left_rotate(&b, 30);
        b = a;
        a = temp;
    }
    let compressed: [u32; 5] = [
        u32_addition!(compressed[0], a),
        u32_addition!(compressed[1], b),
        u32_addition!(compressed[2], c),
        u32_addition!(compressed[3], d),
        u32_addition!(compressed[4], e),
    ];
    compressed
}

fn left_rotate(x: &u32, n: u8) -> u32 {
    (x << n) | (x >> (32 - n))
}

fn concatenate_bytes(bytes: &[u8]) -> u32 {
    bytes
        .iter()
        .map(|byte| *byte as u32)
        .reduce(|con, byte| (con << 8) | byte)
        .expect("Concatenation can't be performed on an empty array.")
}

fn to_bytes<const N: usize>(buffer: &[u32; 5]) -> [u8; N] {
    buffer
        .iter()
        .flat_map(|v| v.to_be_bytes())
        .take(N)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap_or_else(|err: Vec<u8>| panic!("N was {N} when it should not exceed {}", err.len()))
}

impl Display for Sha {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

fn pad<const N: usize>(bytes: &[u8]) -> Vec<u8> {
    let mut decimal = lazy_vector!(bytes.len(), N);
    // Add the binary values to the array.
    bytes
        .iter()
        .enumerate()
        .for_each(|(i, byte)| decimal[i] = *byte);

    // Append a single bit after the last binary.
    decimal[bytes.len()] = 0x80;
    // Get the big endian representation of the length of value.
    let big_endian_rep = (bytes.len() * 8).to_be_bytes();
    big_endian_rep.iter().for_each(|byte| decimal.push(*byte));
    decimal
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sha() {
        let mut sha = Sha::from(&[31, 32, 33]);
        assert_eq!(sha.run(), "40bd001563085fc35165329ea1ff5c5ecbdbbeef");
    }
}
