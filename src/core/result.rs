use std::{fmt::Display, str::from_utf8};
// TODO: Iclude an optional Instant if timers were enabled.
pub struct PasswordMatch {
    password: String,
    algorithm: String,
    length: usize,
    target: String,
}

impl PasswordMatch {
    pub fn from(password: &str, algorithm: String, target: &Vec<u8>) -> Self {
        Self {
            length: password.len(),
            password: password.to_owned(),
            algorithm,
            target: target_to_lower_hex(target),
        }
    }
}

impl Display for PasswordMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Found a match for: {}\nMatch: {}\nLength: {}\nAlgorithm: {}",
            self.target,
            self.password,
            self.length,
            self.algorithm,
        )
    }
}

fn target_to_lower_hex(target: &Vec<u8>) -> String {
    target.iter().map(|dec| format!("{:X}", dec)).collect()
}

fn elapsed_time(elapsed: u64) -> String {
    let seconds = (elapsed % 3600) % 60;
    let minutes = (elapsed % 3600 - seconds) / 60;
    let hours = (elapsed - minutes * 60 + seconds) / 3600;
    format!("H: {}, M: {}, S: {}", hours, minutes, seconds)
}