use serde::{Deserialize, Serialize};
use crate::domain::plans::entities::Plan;
use crate::domain::plans::value_objects::billing_cycle::BillingCycle;
use crate::domain::plans::value_objects::currency::Currency;
use crate::domain::plans::value_objects::price::Price;
use crate::prelude::*;


#[derive(Debug, Deserialize)]
pub struct NewPlanDto {
    pub name: String,
    pub description: Option<String>,
    pub price: Price,
    pub currency: Currency,
    pub billing_cycle: BillingCycle,
    pub stripe_price_id: String,
    pub stripe_product_id: String,
    pub trial_period_days: i32,
    pub active: bool,
}

impl TryFrom<NewPlanDto> for Plan {
    type Error = Error;

    fn try_from(dto: NewPlanDto) -> Result<Self> {
        Ok(Plan::new(
            dto.name,
            dto.description,
            dto.price,
            dto.currency,
            dto.billing_cycle,
            dto.stripe_price_id,
            dto.stripe_product_id,
            dto.trial_period_days,
            dto.active,
        ))
    }
}