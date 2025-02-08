use crate::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq)]
pub struct Price(Decimal);
impl Price {
    pub fn new(value: f64) -> Result<Self> {
        let value = Decimal::try_from(value)
            .map_err(|_| {
                let msg = format!("{}", value);
                Error::InvalidPrice(msg)
            })?;

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
        let value = value
            .parse::<f64>()
            .map_err(|_| Error::InvalidPrice(value.clone()))?;
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
    fn deserialize<D>(deserializer: D) -> std::result::Result<Price, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let price = String::deserialize(deserializer)?;
        Price::try_from(price).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<Decimal> for Price {
    type Error = Error;
    fn try_from(value: Decimal) -> Result<Self> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::ToPrimitive;

    #[test]
    fn test_price_new() {
        let price = Price::new(10.0).unwrap();
        assert_eq!(price.value().to_f64(), Some(10.0_f64));
    }


    #[test]
    fn test_price_deserialize_invalid() {
        let price = serde_json::from_str::<Price>("-10.0");
        assert!(price.is_err());
    }
}
