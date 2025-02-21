use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    #[serde(skip_serializing)]
    id: String,
    email: String,
    name: Option<String>,
}
impl Customer {
    pub fn new(email: String, name: Option<String>) -> Self {
        Self {
            id: "".to_string(),
            email,
            name,
        }
    }
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn email(&self) -> String {
        self.email.clone()
    }
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn update(&mut self, email: Option<String>, name: Option<String>) {
        self.email = email.unwrap_or_else(|| self.email.clone());
        if let Some(name) = name {
            self.name = Some(name);
        }
    }
    pub fn construct(id: String, email: String, name: Option<String>) -> Self {
        Self { id, email, name }
    }
}
