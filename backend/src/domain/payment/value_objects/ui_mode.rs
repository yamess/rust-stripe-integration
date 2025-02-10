use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UiMode {
    Embedded,
    Hosted
}
impl UiMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Embedded => "embedded",
            Self::Hosted => "hosted",
        }
    }
}
impl Display for UiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}