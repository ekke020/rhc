#[derive(Debug, Clone)]
pub enum Mode {
    Dictionary,
    Incremental,
}

impl Mode {
    pub fn from(value: &str) -> Option<Self> {
        match value {
            "dictionary" => Some(Self::Dictionary),
            "incremental" => Some(Self::Incremental),
            _ => None,
        }
    }
}