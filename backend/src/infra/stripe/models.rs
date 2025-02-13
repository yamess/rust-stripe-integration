use serde::{Deserialize, Serialize};
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::product::Product;
use crate::domain::payment::entities::product_price::{ProductPrice, Recurring};
use crate::domain::plans::value_objects::currency::Currency;
use crate::prelude::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct GetCustomerResponse {
    pub data: Vec<Customer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProductResponse {
    pub data: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecurringForm {
    #[serde(rename = "recurring[interval]")]
    pub interval: String,
    #[serde(rename = "recurring[interval_count]")]
    pub interval_count: i32,
    #[serde(rename = "recurring[trial_period_days]")]
    pub trial_period_days: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPriceForm {
    #[serde(skip_serializing)]
    pub id: String,
    pub active: bool,
    pub currency: Currency,
    pub unit_amount: i64,
    pub product: String,
    #[serde(flatten)]
    pub recurring: RecurringForm,
}

impl TryFrom<&ProductPrice> for ProductPriceForm {
    type Error = Error;
    fn try_from(price: &ProductPrice) -> Result<Self> {
        Ok(ProductPriceForm {
            id: price.id().to_string(),
            active: price.active(),
            currency: price.currency().clone(),
            unit_amount: price.unit_amount(),
            product: price.product().to_string(),
            recurring: RecurringForm {
                interval: price.recurring().interval.to_string(),
                interval_count: price.recurring().interval_count,
                trial_period_days: price.recurring().trial_period_days,
            },
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPriceResponse {
    pub data: Vec<ProductPrice>,
}