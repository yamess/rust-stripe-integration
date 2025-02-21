use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerPortalSessionResponse {
    pub id: String,
    pub customer: String,
    pub return_url: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPortalSession {
    customer: String,
    return_url: String,
}
impl CustomerPortalSession {
    pub fn new(customer: String, return_url: String) -> Self {
        Self {
            customer,
            return_url,
        }
    }
    pub fn customer(&self) -> &str {
        &self.customer
    }

    pub fn return_url(&self) -> &str {
        &self.return_url
    }
}
