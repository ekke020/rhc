use sha2::*;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Default)]
pub enum HashType {
    Sha224(String),
    Sha256(String),
    Sha384(String),
    Sha512(String),
    Sha512_224(String),
    Sha512_256(String),
    #[default]
    Empty,
}

impl HashType {
    pub fn get_hash(&self) -> &str {
        match self {
            HashType::Sha224(h) => h,
            HashType::Sha256(h) => h,
            HashType::Sha384(h) => h,
            HashType::Sha512(h) => h,
            HashType::Sha512_224(h) => h,
            HashType::Sha512_256(h) => h,
            HashType::Empty => panic!("Dont match on empty!"),
        }
    }

    pub fn is_match(&self, other: &str) -> bool {
        let hash;
        match self {
            HashType::Empty => return false,
            _ => hash = get_hash(other, self),
        }
        self.get_hash().eq_ignore_ascii_case(&hash)
    }
}
impl fmt::Display for HashType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HashType::Sha224(_) => write!(fmt, "Sha224"),
            HashType::Sha256(_) => write!(fmt, "Sha256"),
            HashType::Sha384(_) => write!(fmt, "Sha384"),
            HashType::Sha512(_) => write!(fmt, "Sha512"),
            HashType::Sha512_224(_) => write!(fmt, "Sha512_224"),
            HashType::Sha512_256(_) => write!(fmt, "Sha512_256"),
            HashType::Empty => write!(fmt, "Empty"),
        }
    }
}

fn get_hash(password: &str, hash_type: &HashType) -> String {
    match hash_type {
        HashType::Sha224(_) => get_sha224(password),
        HashType::Sha256(_) => get_sha256(password),
        HashType::Sha384(_) => get_sha384(password),
        HashType::Sha512(_) => get_sha512(password),
        HashType::Sha512_224(_) => get_sha512_224(password),
        HashType::Sha512_256(_) => get_sha512_256(password),
        HashType::Empty => panic!("No hash value inside Empty"),
    }
}

fn get_sha512(password: &str) -> String {
    let mut sha512 = sha2::Sha512::new();
    sha512.update(password);
    format!("{:X}", sha512.finalize())
}

fn get_sha256(password: &str) -> String {
    let mut sha256 = sha2::Sha256::new();
    sha256.update(password);
    format!("{:X}", sha256.finalize())
}

fn get_sha224(password: &str) -> String {
    let mut sha224 = sha2::Sha224::new();
    sha224.update(password);
    format!("{:X}", sha224.finalize())
}

fn get_sha384(password: &str) -> String {
    let mut sha384 = sha2::Sha384::new();
    sha384.update(password);
    format!("{:X}", sha384.finalize())
}

fn get_sha512_224(password: &str) -> String {
    let mut sha512_224 = sha2::Sha512_224::new();
    sha512_224.update(password);
    format!("{:X}", sha512_224.finalize())
}

fn get_sha512_256(password: &str) -> String {
    let mut sha512_256 = sha2::Sha512_256::new();
    sha512_256.update(password);
    format!("{:X}", sha512_256.finalize())
}
