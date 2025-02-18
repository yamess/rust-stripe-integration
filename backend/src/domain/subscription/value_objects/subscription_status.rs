use std::fmt::Display;
use std::str::FromStr;
use serde::Serialize;
use crate::prelude::*;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionStatus {
    Active,
    Trialing,
    PastDue,
    Canceled,
    Unknown,
}
impl FromStr for SubscriptionStatus {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "trialing" => Ok(Self::Trialing),
            "canceled" => Ok(Self::Canceled),
            "past_due" => Ok(Self::PastDue),
            _ => Ok(Self::Unknown),
        }
    }
}

impl Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Trialing => write!(f, "trialing"),
            Self::Canceled => write!(f, "canceled"),
            Self::PastDue => write!(f, "past_due"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl Serialize for SubscriptionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for SubscriptionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<SubscriptionStatus, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        SubscriptionStatus::from_str(&s).map_err(serde::de::Error::custom)
    }
}