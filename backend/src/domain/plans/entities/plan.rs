use crate::domain::plans::entities::rate::Rate;

#[derive(Debug, Clone)]
pub struct Plan {
    id: i32,
    name: String,
    description: Option<String>,
    stripe_product_id: String,
    rates: Vec<Rate>
}

impl Plan {
    pub fn new(name: String, description: Option<String>,  stripe_product_id: String) -> Self {
        Self {
            id: Default::default(),
            name,
            description,
            stripe_product_id,
            rates: vec![]
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn stripe_product_id(&self) -> &str {
        &self.stripe_product_id
    }
    pub fn rates(&self) -> &Vec<Rate> {
        &self.rates
    }
    pub fn update(&mut self, name: Option<String>, description: Option<String>, stripe_product_id:
                  Option<String>) {
        self.name = name.unwrap_or_else(|| self.name.clone());
        if let Some(description) = description {
            self.description = Some(description);
        }
        self.stripe_product_id = stripe_product_id.unwrap_or_else(|| self.stripe_product_id.clone());
    }
    pub fn add_rate(&mut self, rate: Rate) {
        self.rates.push(rate);
    }
    pub fn construct(
        id: i32, name: String, description: Option<String>, stripe_product_id: String, rates: Vec<Rate>
    ) -> Self {
        Self {
            id,
            name,
            description,
            stripe_product_id,
            rates
        }
    }
}