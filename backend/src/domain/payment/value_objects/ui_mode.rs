use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UiMode {
    Embedded,
    Hosted,
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
impl<'de> serde::Deserialize<'de> for UiMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "embedded" => Ok(Self::Embedded),
            "hosted" => Ok(Self::Hosted),
            _ => Err(serde::de::Error::custom("expected 'embedded' or 'hosted'")),
        }
    }
}
