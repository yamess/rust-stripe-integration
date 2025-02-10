use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingCycle {
    Month(i32),
    Year(i32),
}
impl BillingCycle {
    pub fn new(interval: &str, count: i32) -> Result<Self> {
        match interval {
            "month" => Ok(Self::Month(count)),
            "year" => Ok(Self::Year(count)),
            _ => Err(Error::InvalidBillingCycle("Invalid billing cycle".to_string())),
        }
    }
    pub fn value(&self) -> (&'static str, i32) {
        match self {
            Self::Month(count) => ("month", *count),
            Self::Year(count) => ("year", *count),
        }
    }
}

impl FromStr for BillingCycle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(Error::InvalidBillingCycle("Invalid billing cycle".to_string()));
        }
        let count = parts[0]
            .parse::<i32>()
            .map_err(|_| Error::Parsing("Invalid billing cycle".to_string()))?;
        let interval = parts[1];
        match interval {
            "month" => Ok(Self::Month(count)),
            "year" => Ok(Self::Year(count)),
            _ => Err(Error::InvalidBillingCycle("Invalid billing cycle".to_string())),
        }
    }
}

impl Display for BillingCycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (interval, count) = self.value();
        write!(f, "{}-{}", count, interval)
    }
}

impl Serialize for BillingCycle {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (interval, count) = self.value();
        serializer.serialize_str(&format!("{}-{}", count, interval))
    }
}

impl<'de> Deserialize<'de> for BillingCycle {
    fn deserialize<D>(deserializer: D) -> std::result::Result<BillingCycle, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        BillingCycle::from_str(&s).map_err(serde::de::Error::custom)
    }
}