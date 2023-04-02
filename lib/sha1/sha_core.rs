use std::fmt::Display;

use super::super::{
    common::{pad, to_bytes_32, u32_addition, EXTRACT_ERROR},
    compression::{CompressionSize, Hash, Sha, U20},
};
use super::consts::{StateM1, H_160, K0, K1, K2, K3};
pub struct ShaCore160 {
    value: Option<Vec<u8>>,  // The value that was given
    state: StateM1,          // The current state
    compressed: Option<U20>, // The final hash from the value.
}

impl Sha for ShaCore160 {
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

impl Hash<U20> for ShaCore160 {
    fn load(&mut self, value: &[u8]) {
        self.value = Some(pad::<64>(value));
    }

    fn run(&mut self) {
        let mut buffer = self.state;
        let mut value = self.value.take().unwrap();
        for block in value.chunks_mut(64) {
            let message = prepare_block(block);
            buffer = compression(message, buffer);
        }
        let bytes = to_bytes_32::<20>(&buffer);
        self.compressed = Some(U20::new(&bytes));
    }

    fn extract(&mut self) -> U20 {
        self.compressed.take().expect(EXTRACT_ERROR)
    }
}

impl Default for ShaCore160 {
    fn default() -> Self {
        Self {
            value: None,
            state: H_160,
            compressed: None,
        }
    }
}

fn prepare_block(message: &[u8]) -> [u32; 80] {
    let mut message_schedule: [u32; 80] = [0; 80];
    // Concatenate 4 bytes into a 32bit word.
    message
        .windows(4)
        .step_by(4)
        .enumerate()
        .for_each(|(i, bytes)| {
            message_schedule[i] = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        });
    // Manipulate the bits after index 15.
    for i in 16..80 {
        message_schedule[i] = (message_schedule[i - 3]
            ^ message_schedule[i - 8]
            ^ message_schedule[i - 14]
            ^ message_schedule[i - 16])
            .rotate_left(1);
    }
    message_schedule
}

fn compression(message: [u32; 80], compressed: [u32; 5]) -> [u32; 5] {
    let [mut a, mut b, mut c, mut d, mut e] = compressed;
    for i in 0..80 {
        let [mut f, mut k] = [0, 0];
        match i {
            0..=19 => {
                f = (b & c) ^ ((!b) & d);
                k = K0;
            }
            20..=39 => {
                f = b ^ c ^ d;
                k = K1;
            }
            40..=59 => {
                f = (b & c) ^ (b & d) ^ (c & d);
                k = K2;
            }
            _ => {
                f = b ^ c ^ d;
                k = K3;
            }
        }
        let temp = u32_addition!(a.rotate_left(5), f, e, k, message[i]);
        e = d;
        d = c;
        c = b.rotate_left(30);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prepare_block_returns_correct_values() {
        // pad of "test"
        let padded_value: Vec<u8> = vec![
            0x74, 0x65, 0x73, 0x74, 0x80, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20,
        ];
        let correct: [u32; 80] = [
            1952805748, 2147483648, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 3905611496, 1, 64,
            3516255697, 2, 128, 2737544099, 68, 3516255441, 1180120901, 8, 512, 2360241806, 336,
            2546138774, 425516503, 1925076562, 2054, 851032890, 1180119559, 1594620498, 1702065500,
            1180121031, 8456, 2360467375, 5384, 2083514729, 2513296881, 2295266948, 33124,
            1842120164, 589567163, 2574083911, 1463244250, 1702066548, 1594489818, 2370489751,
            88200, 4031196333, 1558043225, 3554939410, 1702585116, 2594138497, 843147962,
            443001814, 1937066413, 1833912680, 1699897820, 3105786648, 1378536, 803830140,
            2830009825, 3472786440, 8479744, 3431326829, 605337891, 3788056855, 928369223,
            369755153, 279388098, 487021027, 22581440, 283538366, 3098088424,
        ];
        let mutated = prepare_block(&padded_value);
        assert_eq!(mutated, correct);
    }

    #[test]
    fn compression_compresses() {
        // mutated string of "test"
        let chunk: [u32; 80] = [
            1952805748, 2147483648, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 3905611496, 1, 64,
            3516255697, 2, 128, 2737544099, 68, 3516255441, 1180120901, 8, 512, 2360241806, 336,
            2546138774, 425516503, 1925076562, 2054, 851032890, 1180119559, 1594620498, 1702065500,
            1180121031, 8456, 2360467375, 5384, 2083514729, 2513296881, 2295266948, 33124,
            1842120164, 589567163, 2574083911, 1463244250, 1702066548, 1594489818, 2370489751,
            88200, 4031196333, 1558043225, 3554939410, 1702585116, 2594138497, 843147962,
            443001814, 1937066413, 1833912680, 1699897820, 3105786648, 1378536, 803830140,
            2830009825, 3472786440, 8479744, 3431326829, 605337891, 3788056855, 928369223,
            369755153, 279388098, 487021027, 22581440, 283538366, 3098088424,
        ];
        // correct result of compression
        let correct: [u32; 5] = [2840236005, 3434191782, 474744947, 3549555079, 2553265107];
        assert_eq!(compression(chunk, H_160), correct);
    }
}
