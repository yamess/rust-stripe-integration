use std::fmt;
use std::fmt::Write;
use std::str::FromStr;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal_macros::dec;
use serde::{Deserialize, Deserializer, Serialize};
use crate::prelude::*;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Price(i64);
impl Price {
    pub fn new(value: i64) -> Result<Self> {
        Ok(Self(value))
    }
    fn validate(value: i64) -> Result<()> {
        if value >= 0 {
            Ok(())
        } else {
            let msg = format!("Price cannot be negative: {}", value);
            tracing::error!("{}", msg);
            Err(Error::InvalidPrice(msg))
        }
    }
    pub fn value(&self) -> i64 {
        self.0
    }

    pub fn to_decimal(&self) -> Result<Decimal> {
        Decimal::from_i64(self.0).ok_or(Error::InvalidPrice("Invalid decimal value".to_string()))
    }
}

impl TryFrom<i64> for Price {
    type Error = Error;
    fn try_from(value: i64) -> Result<Self> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl TryFrom<Decimal> for Price {
    type Error = Error;
    fn try_from(value: Decimal) -> Result<Self> {
        let value = value.to_i64().ok_or(Error::InvalidPrice("Invalid decimal value".to_string()))?;
        Self::validate(value)?;
        Ok(Self(value))
    }
}

impl TryFrom<String> for Price {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        let value = value.parse::<i64>().map_err(|_| Error::InvalidPrice("Invalid string value".to_string()))?;
        Self::validate(value)?;
        Ok(Self(value))
    }
}

impl Serialize for Price {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Price {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;
        Self::validate(value).map_err(serde::de::Error::custom)?;
        Ok(Self(value))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::ToPrimitive;

    #[test]
    fn test_price_new() {
        let price = Price::new(100);
        assert!(price.is_ok());
    }

    #[test]
    fn test_price_try_from_i64() {
        let price = Price::try_from(100);
        assert!(price.is_ok());
    }

    #[test]
    fn test_price_try_from_decimal() {
        let price = Price::try_from(dec!(100));
        assert!(price.is_ok());
    }

    #[test]
    fn test_price_try_from_string() {
        let price = Price::try_from("100".to_string());
        assert!(price.is_ok());
    }

    #[test]
    fn test_price_try_from_string_invalid() {
        let price = Price::try_from("invalid".to_string());
        assert!(price.is_err());
    }

    #[test]
    fn test_price_try_from_negative() {
        let price = Price::try_from(-100);
        assert!(price.is_err());
    }

    #[test]
    fn test_price_serialize() {
        let price = Price::new(100).unwrap();
        let serialized = serde_json::to_string(&price).unwrap();
        assert_eq!(serialized, "100");
    }

    #[test]
    fn test_price_deserialize() {
        let deserialized: Price = serde_json::from_str("100").unwrap();
        assert_eq!(deserialized.value(), 100);
    }

}
