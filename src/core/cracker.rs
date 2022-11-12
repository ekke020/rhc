use crate::core::hasher::HashType;

const ASCII_TABLE: [char; 95] = [
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2',
    '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
    'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~',
];
#[derive(Clone)]
pub struct Cracked {
    pw_length: usize,
    pw_string: String,
    hash_type: HashType,
}

impl Cracked {
    pub fn new(pw_length: usize, pw_string: String, hash_type: HashType) -> Self {
        Self {
            pw_length,
            pw_string,
            hash_type,
        }
    }

    pub fn print(&self) {
        println!("Hash type: {}", self.hash_type);
        println!("Hash: {}", self.hash_type.get_hash());
        println!("Password: {}", self.pw_string);
        println!("Length: {}", self.pw_length);
    }
}

pub fn run_crack(
    algorithms: &Vec<HashType>,
    pw_length: usize,
    prefix: &String,
    is_cracked: Option<Cracked>,
) -> Option<Cracked> {
    if is_cracked.is_some() {
        return is_cracked;
    } else if pw_length == 0 {
        return algorithms
            .iter()
            .find_map(|hash_type| try_password(prefix, hash_type));
    }
    for char in ASCII_TABLE {
        let mut temp = prefix.clone();
        temp.push(char);
        if let Some(cracked) = run_crack(algorithms, pw_length - 1, &temp, is_cracked.clone()) {
            return Some(cracked);
        }
    }
    None
}

fn try_password(password: &str, hash_type: &HashType) -> Option<Cracked> {
    if hash_type.is_match(password) {
        let cracked = Cracked::new(password.len(), password.to_string(), hash_type.clone());
        return Some(cracked);
    }
    None
}


// #[cfg(test)]
// mod tests {
//     use crate::core::cracker;
//     use crate::core::hasher::HashType;

//     #[test]
//     fn password_matches() {
//         let sha_224 =
//             HashType::Sha224("a7470858e79c282bc2f6adfd831b132672dfd1224c1e78cbf5bcd057".to_owned());
//         let should_be_some = cracker::try_password("12345", &sha_224);
//         assert!(should_be_some.is_some());
//     }
//     #[test]
//     fn password_does_not_match() {
//         let sha_512 = HashType::Sha512("3627909a29c31381a071ec27f7c9ca97726182aed29a7ddd2e54353322cfb30abb9e3a6df2ac2c20fe23436311d678564d0c8d305930575f60e2d3d048184d79".to_owned());
//         let should_be_none = cracker::try_password("1234", &sha_512);
//         assert!(should_be_none.is_none());
//     }
// }
