#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub (super) enum Mode {
    Dictionary,
    Incremental,
}
    // TODO: Redesign this. each mode should hold mode specific values (struct)
impl Mode {
    pub fn from(value: &str) -> Option<Self> {
        match value {
            "dictionary" => Some(Self::Dictionary),
            "incremental" => Some(Self::Incremental),
            _ => None,
        }
    }
}
