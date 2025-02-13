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
    #[serde(skip_serializing)]
    id: String,
    customer: String,
    return_url: String,
    #[serde(skip_serializing)]
    url: String,
}
impl CustomerPortalSession {
    pub fn new(customer: String, return_url: String) -> Self {
        Self {
            id: Default::default(),
            customer,
            return_url,
            url: "".to_string(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn customer(&self) -> &str {
        &self.customer
    }

    pub fn return_url(&self) -> &str {
        &self.return_url
    }
}