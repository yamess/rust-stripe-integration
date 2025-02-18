use std::collections::HashMap;
use std::str::FromStr;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::domain::subscription::entities::Subscription;
use crate::domain::subscription::value_objects::subscription_status::SubscriptionStatus;
use crate::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct PlanObject {
    #[serde(rename(deserialize = "id"))]
    pub price_id: String,
    #[serde(rename(deserialize = "product"))]
    pub product_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewSubscriptionDto {
    pub user_id: Option<uuid::Uuid>,
    #[serde(rename(deserialize = "id"))]
    pub subscription_id: String,  // Subscription id
    #[serde(rename(deserialize = "customer"))]
    pub customer_id: String, // Customer id
    pub plan: PlanObject,
    pub status: SubscriptionStatus,
    pub current_period_end: i64,
    pub cancel_at_period_end: Option<bool>,
}
impl NewSubscriptionDto {
    pub fn into_domain(self) -> Result<Subscription> {
        let user_id = self.user_id.ok_or(Error::BadRequest("User id is required".to_string()))?;
        let has_used_trial = if self.status == SubscriptionStatus::Trialing {
            true
        } else {
            false
        };
        Ok(Subscription::new(
            user_id,
            self.customer_id,
            self.plan.price_id,
            self.plan.product_id,
            self.subscription_id,
            self.status,
            has_used_trial,
            DateTime::<Utc>::from_timestamp(self.current_period_end, 0),
            self.cancel_at_period_end.unwrap_or(false),
        ))
    }
}

//************************************************//
//**************  INVOICE PAID OPS  **************//
//************************************************//
#[derive(Debug, Clone, Deserialize)]
pub struct InvoicePaidEvent {
    #[serde(rename(deserialize = "subscription"))]
    pub subscription_id: String,
    #[serde(rename(deserialize = "customer"))]
    pub customer_id: String,
}