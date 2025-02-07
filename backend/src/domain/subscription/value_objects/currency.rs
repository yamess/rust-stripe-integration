use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    CAD,
}

impl FromStr for Currency {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "USD" => Ok(Self::USD),
            "EUR" => Ok(Self::EUR),
            "GBP" => Ok(Self::GBP),
            "CAD" => Ok(Self::CAD),
            _ => Err(Error::InvalidCurrency(s.to_string())),
        }
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::USD => write!(f, "USD"),
            Self::EUR => write!(f, "EUR"),
            Self::GBP => write!(f, "GBP"),
            Self::CAD => write!(f, "CAD"),
        }
    }
}

impl Serialize for Currency {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Currency {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Currency::from_str(&s).map_err(serde::de::Error::custom)
    }
}