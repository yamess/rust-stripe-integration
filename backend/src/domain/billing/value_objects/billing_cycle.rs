use std::fmt::Display;
use std::str::FromStr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    Semester,
    Yearly,
}

impl BillingCycle {
    pub fn to_days(&self) -> i32 {
        match self {
            Self::Monthly => 30,
            Self::Quarterly => 90,
            Self::Semester => 180,
            Self::Yearly => 365,
        }
    }
    pub fn interval(&self) -> (&'static str, i32) {
        match self {
            Self::Monthly => ("month", 1),
            Self::Quarterly => ("month", 3),
            Self::Semester => ("month" , 6),
            Self::Yearly => ("year", 1),
        }
    }
}

impl FromStr for BillingCycle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "monthly" => Ok(Self::Monthly),
            "quarterly" => Ok(Self::Quarterly),
            "semester" => Ok(Self::Semester),
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
            Self::Semester => write!(f, "semester"),
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