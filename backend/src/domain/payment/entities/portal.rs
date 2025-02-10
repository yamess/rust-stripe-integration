use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPortalSession {
    #[serde(skip_serializing)]
    id: String,
    customer: String,
    return_url: String,
    #[serde(skip_serializing)]
    url: String,
}