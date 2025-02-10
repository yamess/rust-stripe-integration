use serde::{Deserialize, Serialize};
use crate::domain::plans::value_objects::currency::Currency;

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
    unit_amount: i32,
    product: String,
    recurring: Recurring
}

