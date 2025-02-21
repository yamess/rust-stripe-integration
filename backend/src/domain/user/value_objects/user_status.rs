use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Inactive,
    Banned,
    Suspended,
    Pending,
}

impl UserStatus {
    pub fn value(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Banned => "banned",
            Self::Suspended => "suspended",
            Self::Pending => "pending",
        }
    }
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<String> for UserStatus {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            "banned" => Ok(UserStatus::Banned),
            "suspended" => Ok(UserStatus::Suspended),
            "pending" => Ok(UserStatus::Pending),
            _ => Err(Error::InvalidUserStatus(value.to_string())),
        }
    }
}

impl Serialize for UserStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.value())
    }
}

impl<'de> serde::Deserialize<'de> for UserStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        UserStatus::try_from(s).map_err(serde::de::Error::custom)
    }
}
