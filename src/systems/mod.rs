pub mod cracker;
pub mod filereader;
pub mod hasher;
pub mod password_info;
pub mod password_matcher;
pub mod printer;
pub mod input;

pub mod macros {
    macro_rules! lazy_array {
        ($start:expr, $size:expr, u8) => {{
            let mut temp_arr: [u8; $size] = [0; $size];
            for i in 0..$size {
                temp_arr[i] = (i + $start) as u8;
            }
            temp_arr
        }};
        ($start:expr, $size:expr, char) => {{
            let mut temp_arr: [char; $size] = ['a'; $size];
            for i in 0..$size {
                temp_arr[i] = ((i + $start) as u8 as char);
            }
            temp_arr
        }};
    }
    macro_rules! lazy_vector {
        ($start:expr, $size:expr, char) => {{
            let mut temp_vec: Vec<char> = Vec::new();
            for i in 0..$size {
                temp_vec.push((i + $start) as u8 as char);
            }
            temp_vec
        }};
    }

    pub(crate) use {lazy_array, lazy_vector};
}
