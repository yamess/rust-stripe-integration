use std::fmt::Display;
use std::str::FromStr;
use serde::Serialize;
use crate::prelude::*;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Trialing,
    Unpaid,
}
impl FromStr for SubscriptionStatus {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "canceled" => Ok(Self::Canceled),
            "incomplete" => Ok(Self::Incomplete),
            "incomplete_expired" => Ok(Self::IncompleteExpired),
            "past_due" => Ok(Self::PastDue),
            "trialing" => Ok(Self::Trialing),
            "unpaid" => Ok(Self::Unpaid),
            _ => Err(Error::InvalidSubscriptionStatus(s.to_string())),
        }
    }
}

impl Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Canceled => write!(f, "canceled"),
            Self::Incomplete => write!(f, "incomplete"),
            Self::IncompleteExpired => write!(f, "incomplete_expired"),
            Self::PastDue => write!(f, "past_due"),
            Self::Trialing => write!(f, "trialing"),
            Self::Unpaid => write!(f, "unpaid"),
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