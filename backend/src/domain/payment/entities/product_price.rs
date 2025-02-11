use serde::{Deserialize, Serialize};
use crate::domain::plans::value_objects::currency::Currency;
use crate::domain::plans::value_objects::price::Price;

#[derive(Debug, Serialize, Deserialize)]
pub struct Recurring {
    interval: String,
    interval_count: i32,
    trial_period_days: Option<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPrice {
    #[serde(skip_serializing)]
    id: String,
    active: bool,
    currency: Currency,
    unit_amount: Price,
    product: String,
    recurring: Recurring
}
impl ProductPrice {
    pub fn new(active: bool, currency: Currency, unit_amount: Price, product: String, recurring: Recurring) -> Self {
        Self {
            id: "".to_string(),
            active,
            currency,
            unit_amount,
            product,
            recurring
        }
    }
}