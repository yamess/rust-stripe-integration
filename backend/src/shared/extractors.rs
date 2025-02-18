use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::prelude::*;

pub fn extract_bool(data: &Value, key: &str) -> Result<bool> {
    data.pointer(&format!("/{}", key))
        .and_then(|v| v.as_bool())
        .ok_or(Error::BadRequest(format!("Missing or Invalid `{}`", key)))
}

pub fn extract_string(data: &Value, key: &str) -> Result<String> {
        data.pointer(&format!("/{}", key))
            .and_then(|v| v.as_str())
            .map(String::from)
            .ok_or(Error::BadRequest(format!("Missing or Invalid `{}`", key)))
    }

pub fn extract_number(data: &Value, key: &str) -> Result<i64> {
    data.pointer(&format!("/{}", key))
        .and_then(|v| v.as_i64())
        .ok_or(Error::BadRequest(format!("Missing or Invalid `{}`", key)))
}

pub fn extract_timestamp(data: &Value, key: &str) -> Result<DateTime<Utc>> {
    let timestamp: i64 = data.pointer(&format!("/{}", key))
        .and_then(|v| v.as_i64())
        .ok_or(Error::BadRequest(format!("Missing or Invalid `{}`", key)))?;

    // let timestamp: i64 = extract_string(data, key)?
    //     .parse()
    //     .map_err(|_| Error::BadRequest(format!("Invalid timestamp `{}`", key)))?;

    DateTime::<Utc>::from_timestamp(timestamp, 0)
        .ok_or_else(|| Error::BadRequest(format!("Invalid timestamp `{}`", key)))
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_extract_string() {
        let data = json!({
            "name": "John Doe",
            "address": {
                "city": "New York"
            }
        });

        let key = "name";
        let result = extract_string(&data, key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "John Doe");

        let key = "address/city";
        let result = extract_string(&data, key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "New York");

        let key = "email";
        let result = extract_string(&data, key);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_number() {
        let data = json!({
            "amount": 100
        });

        let key = "amount";
        let result = extrac_number(&data, key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);

        let data = json!({
            "amount": "invalid"
        });

        let key = "amount";
        let result = extrac_number(&data, key);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_timestamp() {
        let data = json!({
            "timestamp": 1619680000
        });

        let key = "timestamp";
        let result = extract_timestamp(&data, key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DateTime::<Utc>::from_timestamp(1619680000, 0).unwrap());

        let data = json!({
            "timestamp": "invalid"
        });

        let key = "timestamp";
        let result = extract_timestamp(&data, key);
        assert!(result.is_err());
    }
}