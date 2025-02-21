use crate::domain::subscription::service::SignatureVerificationService;
use crate::prelude::{Result, Error};
use hmac::digest::InvalidBufferSize;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct StripeSignatureVerificationService {
    secret: String,
    tolerance: u64,
}
impl StripeSignatureVerificationService {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
            tolerance: 300,
        }
    }
}

impl SignatureVerificationService for StripeSignatureVerificationService {
    fn verify(&self, payload: &str, signature_header: &str) -> Result<()> {
        let mut timestamp = None;
        let mut signature = None;

        for part in signature_header.split(',') {
            let kv: Vec<&str> = part.splitn(2, '=').collect();
            if kv.len() != 2 {
                return Err(Error::InvalidSignature(format!(
                    "Invalid signature format in part: {}",
                    part
                )));
            }

            match kv[0].trim() {
                "t" => timestamp = Some(kv[1].trim()),
                "v1" => signature = Some(kv[1].trim()),
                _ => continue,
            }
        }

        let timestamp = timestamp.ok_or(Error::InvalidSignature(
            "Missing timestamp in signature header".to_string(),
        ))?;
        let signature = signature.ok_or(Error::InvalidSignature(
            "Missing v1 signature in signature header".to_string(),
        ))?;

        // Convert timestamp to u64 and verify it's not too old
        let timestamp_secs: u64 = timestamp
            .parse()
            .map_err(|_| Error::InvalidSignature("Invalid timestamp format".to_string()))?;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::InvalidSignature("System time error".to_string()))?
            .as_secs();

        if current_time < timestamp_secs || current_time - timestamp_secs > self.tolerance {
            return Err(Error::InvalidSignature(format!(
                "Timestamp {} outside tolerance window (current: {})",
                timestamp_secs, current_time
            )));
        }

        // Construct the signed payload per Stripe's specification
        let signed_payload = format!("{}.{}", timestamp, payload);

        // Compute HMAC-SHA256 signature
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes()).map_err(|e| {
            tracing::error!("HMAC initialization error: {}", e);
            Error::InternalError
        })?;
        mac.update(signed_payload.as_bytes());
        let computed_signature = hex::encode(mac.finalize().into_bytes());

        // Compare signatures (Stripe uses hex-encoded strings)
        if computed_signature != signature {
            return Err(Error::InvalidSignature(format!(
                "Signature mismatch (expected: {}, got: {})",
                computed_signature, signature
            )));
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Error;

    #[test]
    fn test_verify_signature() {
        let service = StripeSignatureVerificationService::new("whsec_test_secret");

        let payload = r#"{"id":"evt_test_event"}"#;
        let timestamp = "1612210817";

        let signed_payload = format!("{}.{}", timestamp, payload);

        let mut mac = HmacSha256::new_from_slice("whsec_test_secret".as_bytes()).unwrap();
        mac.update(signed_payload.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        let signature_header = format!("t={},v1={}", timestamp, signature);

        assert!(service.verify(payload, &signature_header).is_ok());

        // Test with invalid payload

        // let payload = r#"{"id":"evt_test_event"}"#;
        //
        // let signed_payload = format!("{}.{}", timestamp, payload);
        //
        // let mut mac = HmacSha256::new_from_slice("whsec_test
        //
        //     _secret".as_bytes()).unwrap();
        //
        // mac.update(signed_payload.as_bytes());
        //
        // let signature = hex::encode(mac.finalize().into_bytes());
        //
        // let signature_header = format!("t={},v1={}", timestamp, signature);
        //
        // assert_eq!(
        //     service.verify(payload, &signature_header),
        //     Err(Error::InvalidSignature("Signature mismatch".to_string()))
        // );
    }
}