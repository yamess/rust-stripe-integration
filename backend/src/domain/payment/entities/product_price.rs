use serde::{Deserialize, Serialize};
use stripe::Recurring;
use crate::domain::plans::value_objects::currency::Currency;

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

