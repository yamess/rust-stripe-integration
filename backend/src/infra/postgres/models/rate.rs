use std::str::FromStr;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rust_decimal::Decimal;
use crate::domain::plans::entities::rate::Rate;
use crate::domain::plans::value_objects::billing_cycle::BillingCycle;
use crate::domain::plans::value_objects::currency::Currency;
use crate::domain::plans::value_objects::price::Price;
use crate::infra::postgres::models::plan::PlanModel;
use crate::prelude::*;
use crate::schema::rates;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = rates, check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(PlanModel))]
pub struct RateModel {
    id: i32,
    stripe_price_id: String,
    plan_id: i32,
    currency: String,
    amount: Decimal,
    billing_cycle: String,
    active: bool,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}
impl TryFrom<RateModel> for Rate {
    type Error = Error;

    fn try_from(model: RateModel) -> Result<Self> {
        Ok(Rate::construct(
            model.id,
            model.stripe_price_id,
            model.plan_id,
            Currency::from_str(&model.currency)?,
            Price::try_from(model.amount)?,
            BillingCycle::from_str(&model.billing_cycle)?,
            model.active,
        ))
    }
}


#[derive(Debug, Insertable)]
#[diesel(table_name = rates, check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(PlanModel))]
pub struct CreateRateModel {
    pub stripe_price_id: String,
    pub plan_id: i32,
    pub currency: String,
    pub amount: Decimal,
    pub billing_cycle: String,
    pub active: bool,
}
impl TryFrom<&Rate> for CreateRateModel {
    type Error = Error;

    fn try_from(rate: &Rate) -> Result<Self> {
        Ok(Self {
            stripe_price_id: rate.stripe_price_id().to_string(),
            plan_id: rate.plan_id(),
            currency: rate.currency().to_string(),
            amount: rate.amount().to_decimal()?,
            billing_cycle: rate.billing_cycle().to_string(),
            active: rate.active(),
        })
    }
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = rates, check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(PlanModel))]
pub struct UpdateRateModel {
    currency: String,
    amount: Decimal,
    billing_cycle: String,
    active: bool
}
impl TryFrom<&Rate> for UpdateRateModel {
    type Error = Error;

    fn try_from(rate: &Rate) -> Result<Self> {
        Ok(Self {
            currency: rate.currency().to_string(),
            amount: rate.amount().to_decimal()?,
            billing_cycle: rate.billing_cycle().to_string(),
            active: rate.active(),
        })
    }
}