use crate::prelude::*;

const ASCII_TABLE: [char; 95] = [
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2',
    '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
    'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~',
];

pub fn crack(length: usize, start_char: &char) -> Option<PasswordInfo> {
    run_crack(length, &start_char.to_string(), None)
}

fn run_crack(
    pw_length: usize,
    prefix: &String,
    is_cracked: Option<PasswordInfo>,
) -> Option<PasswordInfo> {
    if is_cracked.is_some() {
        return is_cracked;
    } else if pw_length == 0 {
        return MATCHER_INFO.try_password(prefix);
    }
    for char in ASCII_TABLE {
        let mut temp = prefix.clone();
        temp.push(char);
        if let Some(cracked) = run_crack(pw_length - 1, &temp, is_cracked.clone()) {
            return Some(cracked);
        }
    }
    None
}


// pub fn testing() {
//     let mut password_length = 1;
//     let mut crack_result = None;
//     while let None = crack_result {
//         let test = print_all(&mut String::new(), ASCII_TABLE.len(), password_length, None);
//         match test {
//             Some(t) => {
//                 crack_result = Some(t);
//             }
//             None => password_length += 1,
//         }
//     }
//     println!("{}", crack_result.unwrap().to_string());
// }

// fn print_all(
//     prefix: &mut String,
//     n: usize,
//     k: i32,
//     is_cracked: Option<PW::PasswordInfo>,
// ) -> Option<PW::PasswordInfo> {
//     if is_cracked.is_some() {
//         println!("Hello");
//         return is_cracked;
//     }
//     if k == 0 {
//         return MATCHER_INFO.try_password(prefix);
//     }
//     let mut cracked = None;
//     for i in 0..n {
//         let char = ASCII_TABLE[i];
//         let mut temp = prefix.clone();
//         temp.push(char);
//         cracked = print_all(&mut temp, n, k - 1, is_cracked.clone());
//         if cracked.is_some() {
//             return cracked;
//         }
//     }
//     cracked
// }

// fn get_ascii() -> [u8; 95] {
//     let ascii_num = macros::lazy_array!(32, 95, u8);
//     let ascii_num_extended_one = macros::lazy_array!(161, 12, u8);
//     let ascii_num_extended_two = macros::lazy_array!(174, 82, u8);
//     let full_ascii: [u8; 189] = ascii_num
//         .iter()
//         .chain(&ascii_num_extended_one)
//         .chain(&ascii_num_extended_two)
//         .map(|code| *code)
//         .collect::<Vec<u8>>()
//         .try_into()
//         .unwrap();
//     ascii_num
// }
