use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    #[serde(skip_serializing)]
    id: String,
    email: String,
    name: String
}
impl Customer {
    pub fn new(email: String, name: String) -> Self {
        Self {
            id: "".to_string(),
            email,
            name
        }
    }
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn email(&self) -> String {
        self.email.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn update(&mut self, email: Option<String>, name: Option<String>) {
        self.email = email.unwrap_or_else(|| self.email.clone());
        self.name = name.unwrap_or_else(|| self.name.clone());
    }
    pub fn construct(id: String, email: String, name: String) -> Self {
        Self {
            id,
            email,
            name
        }
    }
}

