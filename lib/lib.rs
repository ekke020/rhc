#![allow(warnings)]
pub mod sha1;
pub mod sha2;
mod common;
mod wrapper;
mod compression;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum AlgorithmType {
    Sha1,
    Sha2_224,
    Sha2_256,
    Sha2_384,
    Sha2_512,
    Sha2_512_224,
    Sha2_512_256,
}

impl AlgorithmType {
    pub fn from(value: &str) -> Option<Self> {
        match value {
            "sha1" => Some(AlgorithmType::Sha1),
            "sha2_224" => Some(AlgorithmType::Sha2_224),
            "sha2_256" => Some(AlgorithmType::Sha2_256),
            "sha2_384" => Some(AlgorithmType::Sha2_384),
            "sha2_512" => Some(AlgorithmType::Sha2_512),
            "sha2_512_224" => Some(AlgorithmType::Sha2_512_224),
            "sha2_512_256" => Some(AlgorithmType::Sha2_512_256),
            _ => None,
        }
    }

    pub fn generate_algorithm(&self) -> Box<dyn Algorithm> {
        match self {
            AlgorithmType::Sha2_224 => Box::new(sha2::Sha224::new()),
            AlgorithmType::Sha2_256 => Box::new(sha2::Sha256::new()),
            AlgorithmType::Sha2_384 => Box::new(sha2::Sha384::new()),
            AlgorithmType::Sha2_512 => Box::new(sha2::Sha512::new()),
            AlgorithmType::Sha2_512_224 => Box::new(sha2::Sha512_224::new()),
            AlgorithmType::Sha2_512_256 => Box::new(sha2::Sha512_256::new()),
            AlgorithmType::Sha1 => Box::new(sha1::Sha160::new()),
        }
    }
}

pub trait Algorithm: Display {
    fn populate(&mut self, data: &[u8]);

    fn execute(&mut self);

    fn compare(&mut self, target: &Vec<u8>) -> bool;
}

impl Algorithm for sha1::Sha160 {
    fn populate(&mut self, data: &[u8]) {
        self.load(data);
    }

    fn execute(&mut self) {
        self.run();
    }

    fn compare(&mut self, target: &Vec<u8>) -> bool {
        let value = self.extract();
        target[..] == value
    }
}

impl Algorithm for sha2::Sha224 {
    fn populate(&mut self, data: &[u8]) {
        self.load(data);
    }

    fn execute(&mut self) {
        self.run();
    }

    fn compare(&mut self, target: &Vec<u8>) -> bool {
        let value = self.extract();
        target[..] == value
    }
}

impl Algorithm for sha2::Sha256 {
    fn populate(&mut self, data: &[u8]) {
        self.load(data);
    }

    fn execute(&mut self) {
        self.run();
    }

    fn compare(&mut self, target: &Vec<u8>) -> bool {
        let value = self.extract();
        target[..] == value
    }
}

impl Algorithm for sha2::Sha384 {
    fn populate(&mut self, data: &[u8]) {
        self.load(data);
    }

    fn execute(&mut self) {
        self.run();
    }

    fn compare(&mut self, target: &Vec<u8>) -> bool {
        let value = self.extract();
        target[..] == value
    }
}

impl Algorithm for sha2::Sha512 {
    fn populate(&mut self, data: &[u8]) {
        self.load(data);
    }

    fn execute(&mut self) {
        self.run();
    }

    fn compare(&mut self, target: &Vec<u8>) -> bool {
        let value = self.extract();
        target[..] == value
    }
}

impl Algorithm for sha2::Sha512_224 {
    fn populate(&mut self, data: &[u8]) {
        self.load(data);
    }

    fn execute(&mut self) {
        self.run();
    }

    fn compare(&mut self, target: &Vec<u8>) -> bool {
        let value = self.extract();
        target[..] == value
    }
}

impl Algorithm for sha2::Sha512_256 {
    fn populate(&mut self, data: &[u8]) {
        self.load(data);
    }

    fn execute(&mut self) {
        self.run();
    }

    fn compare(&mut self, target: &Vec<u8>) -> bool {
        let value = self.extract();
        target[..] == value
    }
}
