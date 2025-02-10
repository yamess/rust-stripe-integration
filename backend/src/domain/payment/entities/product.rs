use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(skip_serializing)]
    id: String,
    name: String,
    active: bool
}
impl Product {
    pub fn new(name: String) -> Self {
        Self {
            id: "".to_string(),
            name,
            active: true
        }
    }
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn active(&self) -> bool {
        self.active
    }
    pub fn update(&mut self, name: Option<String>, active: Option<bool>) {
        self.name = name.unwrap_or_else(|| self.name.clone());
        self.active = active.unwrap_or_else(|| self.active);
    }
    pub fn construct(id: String, name: String, active: bool) -> Self {
        Self {
            id,
            name,
            active
        }
    }
}
