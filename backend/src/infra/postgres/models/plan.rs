use std::str::FromStr;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use diesel;
use rust_decimal::Decimal;

use crate::domain::billing::entities::plan::Plan;
use crate::domain::billing::value_objects::{price::Price, currency::Currency};
use crate::domain::billing::value_objects::billing_cycle::BillingCycle;
use crate::schema;
use crate::prelude::*;


#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::plans, check_for_backend(diesel::pg::Pg))]
pub struct PlanModel {
    id: i32,
    name: String,
    description: Option<String>,
    price: Decimal,
    currency: String,
    billing_cycle: String,
    stripe_price_id: String,
    stripe_product_id: String,
    trial_period_days: i32,
    active: bool,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}
impl TryFrom<&Plan> for PlanModel {
    type Error = Error;

    fn try_from(plan: &Plan) -> Result<Self> {
        Ok(Self {
            id: plan.id(),
            name: plan.name().to_string(),
            description: plan.description().map(String::from),
            price: plan.price().value(),
            currency: plan.currency().to_string(),
            billing_cycle: plan.billing_cycle().to_string(),
            stripe_price_id: plan.stripe_price_id().to_string(),
            stripe_product_id: plan.stripe_product_id().to_string(),
            trial_period_days: plan.trial_period_days(),
            active: plan.active(),
            created_at: plan.created_at(),
            updated_at: plan.updated_at(),
        })
    }
}
impl TryFrom<PlanModel> for Plan {
    type Error = Error;

    fn try_from(model: PlanModel) -> Result<Self> {
        Ok(Plan::construct(
            model.id,
            model.name,
            model.description,
            Price::try_from(model.price)?,
            Currency::from_str(&model.currency)?,
            BillingCycle::from_str(&model.billing_cycle)?,
            model.stripe_price_id,
            model.stripe_product_id,
            model.trial_period_days,
            model.active,
            model.created_at,
            model.updated_at,
        ))
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::plans, check_for_backend(diesel::pg::Pg))]
pub struct NewPlanModel {
    name: String,
    description: Option<String>,
    price: Decimal,
    currency: String,
    billing_cycle: String,
    stripe_price_id: String,
    stripe_product_id: String,
    trial_period_days: i32,
    active: bool,
}
impl TryFrom<&Plan> for NewPlanModel {
    type Error = Error;

    fn try_from(plan: &Plan) -> Result<Self> {
        Ok(Self {
            name: plan.name().to_string(),
            description: plan.description().map(String::from),
            price: plan.price().value(),
            currency: plan.currency().to_string(),
            billing_cycle: plan.billing_cycle().to_string(),
            stripe_price_id: plan.stripe_price_id().to_string(),
            stripe_product_id: plan.stripe_product_id().to_string(),
            trial_period_days: plan.trial_period_days(),
            active: plan.active(),
        })
    }
}


#[derive(Debug, Clone,AsChangeset)]
#[diesel(table_name = schema::plans, check_for_backend(diesel::pg::Pg))]
pub struct UpdatePlanModel {
    name: String,
    description: Option<String>,
    price: Decimal,
    currency: String,
    billing_cycle: String,
    stripe_price_id: String,
    stripe_product_id: String,
    trial_period_days: i32,
    active: bool,
    updated_at: Option<DateTime<Utc>>,
}
impl TryFrom<&Plan> for UpdatePlanModel {
    type Error = Error;

    fn try_from(plan: &Plan) -> Result<Self> {
        Ok(Self {
            name: plan.name().to_string(),
            description: plan.description().map(String::from),
            price: plan.price().value(),
            currency: plan.currency().to_string(),
            billing_cycle: plan.billing_cycle().to_string(),
            stripe_price_id: plan.stripe_price_id().to_string(),
            stripe_product_id: plan.stripe_product_id().to_string(),
            trial_period_days: plan.trial_period_days(),
            active: plan.active(),
            updated_at: plan.updated_at(),
        })
    }
}