use std::fmt::Display;
use serde::Serialize;

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
impl Default for UiMode {
    fn default() -> Self {
        Self::Hosted
    }
}
impl Display for UiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl Serialize for UiMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
