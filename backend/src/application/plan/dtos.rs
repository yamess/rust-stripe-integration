use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPlan {
    pub name: String,
    pub description: Option<String>,
}