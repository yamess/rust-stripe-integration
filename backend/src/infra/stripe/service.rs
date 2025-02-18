use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::domain::subscription::service::SignatureVerificationService;
use crate::prelude::Error;


type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct StripeSignatureVerificationService {
    secret: String
}
impl StripeSignatureVerificationService {
    pub fn new(secret: &str) -> Self {
        Self { secret: secret.to_string() }
    }
}

impl SignatureVerificationService for StripeSignatureVerificationService {
    fn verify(&self, body: &[u8], signature: &str) -> crate::prelude::Result<()> {
        let mut signed_payload = String::new();
        let mut expected_signature = String::new();

        for part in signature.split(',') {
            let kv: Vec<&str> = part.splitn(2, '=').collect();
            if kv.len() == 2 {
                match kv[0] {
                    "t" => signed_payload = kv[1].to_string(),
                    "v1" => expected_signature = kv[1].to_string(),
                    _ => {}
                }
            }
        }

        signed_payload.push('.');
        signed_payload.push_str(&String::from_utf8_lossy(body));

        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .map_err(|_| Error::InternalError)?;
        mac.update(signed_payload.as_bytes());
        let computed_signature = hex::encode(mac.finalize().into_bytes());

        if computed_signature == expected_signature {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }

}