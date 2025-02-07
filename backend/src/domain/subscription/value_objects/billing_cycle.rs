use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    Yearly,
}

impl FromStr for BillingCycle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "monthly" => Ok(Self::Monthly),
            "quarterly" => Ok(Self::Quarterly),
            "yearly" => Ok(Self::Yearly),
            _ => Err(Error::InvalidBillingCycle(s.to_string())),
        }
    }
}

impl Display for BillingCycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Monthly => write!(f, "monthly"),
            Self::Quarterly => write!(f, "quarterly"),
            Self::Yearly => write!(f, "yearly"),
        }
    }
}

impl Serialize for BillingCycle {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for BillingCycle {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        BillingCycle::from_str(&s).map_err(serde::de::Error::custom)
    }
}