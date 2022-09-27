
//! Lazily initializes a vector with 0x00.
//! 
//! The sha implementation requires that each byte 
//! 
macro_rules! lazy_vector {
    ($length:expr, $size:expr) => {{
        let m: usize = $length / $size;
        let capacity: usize = (m + 1) * $size;
        let mut temp: Vec<u8> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            temp.push(0x00);
        }
        temp
    }};
}