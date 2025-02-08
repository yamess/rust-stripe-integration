use std::sync::Arc;

#[derive(Clone)]
pub struct StripeClient {
    http: Arc<reqwest::Client>,
    secret_key: String,
}
impl StripeClient {
    pub fn new(secret_key: String, http_client: Arc<reqwest::Client>) -> Self {
        Self {
            http: http_client,
            secret_key,
        }
    }
}