use serde::{Deserialize, Serialize};
use crate::domain::plans::value_objects::currency::Currency;
use crate::domain::plans::value_objects::price::Price;

#[derive(Debug, Serialize, Deserialize)]
pub struct Recurring {
    // #[serde( = "recurring[interval]")]
    pub interval: String,
    // #[serde(rename = "recurring[interval_count]")]
    pub interval_count: i32,
    // #[serde(rename = "recurring[trial_period_days]")]
    pub trial_period_days: Option<i32>
}
impl Recurring {
    pub fn new(interval: String, interval_count: i32, trial_period_days: Option<i32>) -> Self {
        Self {
            interval,
            interval_count,
            trial_period_days
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPrice {
    #[serde(skip_serializing)]
    id: String,
    active: bool,
    currency: Currency,
    unit_amount: i64,
    product: String,
    // #[serde(flatten)]
    recurring: Recurring
}
impl ProductPrice {
    pub fn new(active: bool, currency: Currency, unit_amount: i64, product: String, recurring:
    Recurring) -> Self {
        Self {
            id: "".to_string(),
            active,
            currency,
            unit_amount,
            product,
            recurring
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn unit_amount(&self) -> i64 {
        self.unit_amount
    }

    pub fn product(&self) -> &str {
        &self.product
    }

    pub fn recurring(&self) -> &Recurring {
        &self.recurring
    }

    pub fn construct(
        id: String,
        active: bool,
        currency: Currency,
        unit_amount: i64,
        product: String,
        recurring: Recurring
    ) -> Self {
        Self {
            id,
            active,
            currency,
            unit_amount,
            product,
            recurring
        }
    }
}