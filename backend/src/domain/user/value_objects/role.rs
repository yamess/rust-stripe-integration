use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
    Guest,
    Super,
}

impl Role {
    pub fn value(&self) -> &str {
        match self {
            Self::Admin => "admin",
            Self::User => "user",
            Self::Guest => "guest",
            Self::Super => "super",
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<String> for Role {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            "guest" => Ok(Role::Guest),
            "super" => Ok(Role::Super),
            _ => Err(Error::InvalidRole(value.to_string())),
        }
    }
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.value())
    }
}

impl<'de> serde::Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Role::try_from(s).map_err(serde::de::Error::custom)
    }
}
