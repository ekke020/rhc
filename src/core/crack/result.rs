use std::fmt::Display;
// TODO: Iclude an optional Instant if timers were enabled.
pub struct PasswordMatch {
    password: String,
    algorithm: String,
    length: usize,
    target: Vec<u8>,
}

impl PasswordMatch {
    pub fn from(password: String, algorithm: String, target: Vec<u8>) -> Self {
        Self {
            length: password.len(),
            password,
            algorithm,
            target,
        }
    }
}

impl Display for PasswordMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Found a match for: {}\nMatch: {}\nLength: {}\nAlgorithm: {}",
            target_to_lower_hex(&self.target),
            self.password,
            self.length,
            self.algorithm
        )
    }
}

fn target_to_lower_hex(target: &Vec<u8>) -> String {
    target.iter().map(|dec| format!("{:X}", dec)).collect()
}
