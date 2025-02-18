use crate::prelude::*;

pub trait SignatureVerificationService: Send + Sync {
    fn verify(&self, body: &[u8], signature: &str) -> Result<()>;
}