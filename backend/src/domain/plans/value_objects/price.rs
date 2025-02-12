use std::fmt;
use std::fmt::Write;
use std::str::FromStr;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Deserializer, Serialize};
use crate::prelude::*;


#[derive(Debug, Clone, PartialEq)]
pub struct Price(Decimal);
impl Price {
    pub fn new(value: Decimal) -> Result<Self> {
        Self::validate(value)?;
        Ok(Self(value))
    }
    fn validate(value: Decimal) -> Result<()> {
        if value >= dec!(0.0) {
            Ok(())
        } else {
            let msg = format!("Price cannot be negative: {}", value);
            tracing::error!("{}", msg);
            Err(Error::InvalidPrice(msg))
        }
    }
    pub fn value(&self) -> Decimal {
        self.0
    }
}

impl TryFrom<String> for Price {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        let value = Decimal::from_str(&value).map_err(|_| Error::InvalidPrice(value.clone()))?;
        Self::new(value)
    }
}

impl TryFrom<i64>  for Price {
    type Error = Error;
    fn try_from(value: i64) -> Result<Self> {
        let value = Decimal::new(value, 2);
        Self::new(value)
    }
}

impl TryFrom<Decimal> for Price {
    type Error = Error;
    fn try_from(value: Decimal) -> Result<Self> {
        Self::new(value)
    }
}

impl Serialize for Price {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Price {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PriceVisitor;

        impl<'de> serde::de::Visitor<'de> for PriceVisitor {
            type Value = Price;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or an integer representing a price")
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Price, E>
            where
                E: serde::de::Error,
            {
                Price::try_from(value.to_string()).map_err(serde::de::Error::custom)
            }

            fn visit_i64<E>(self, value: i64) -> std::result::Result<Price, E>
            where
                E: serde::de::Error,
            {
                Price::try_from(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(PriceVisitor)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::ToPrimitive;

    #[test]
    fn test_price_new() {
        let price = Price::new(dec!(10.0)).unwrap();
        assert_eq!(price.value().to_f64().unwrap(), 10.0);
    }

    #[test]
    fn test_price_try_from_string() {
        let price = Price::try_from("10.0".to_string()).unwrap();
        assert_eq!(price.value().to_f64().unwrap(), 10.0);
    }

    #[test]
    fn test_price_try_from_i64() {
        let price = Price::try_from(10).unwrap();
        assert_eq!(price.value().to_f64().unwrap(), 10.0);
    }

    #[test]
    fn test_price_try_from_decimal() {
        let price = Price::try_from(dec!(10.0)).unwrap();
        assert_eq!(price.value().to_f64().unwrap(), 10.0);
    }

    #[test]
    fn test_price_serialize() {
        let price = Price::try_from(dec!(10.0)).unwrap();
        let serialized = serde_json::to_string(&price).unwrap();
        assert_eq!(serialized, "\"10\"");
    }

    #[test]
    fn test_price_deserialize() {
        let deserialized: Price = serde_json::from_str("\"10\"").unwrap();
        assert_eq!(deserialized.value().to_f64().unwrap(), 10.0);
    }
}
