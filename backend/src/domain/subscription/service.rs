use crate::prelude::*;

pub trait SignatureVerificationService: Send + Sync {
    fn verify(&self, body: &str, signature: &str) -> Result<()>;
}
