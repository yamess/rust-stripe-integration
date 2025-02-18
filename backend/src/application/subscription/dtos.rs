use std::collections::HashMap;
use std::str::FromStr;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;
use crate::domain::subscription::entities::Subscription;
use crate::domain::subscription::value_objects::subscription_status::SubscriptionStatus;
use crate::schema::subscriptions::{current_period_end, stripe_subscription_id};
use crate::schema::users::stripe_customer_id;
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
    pub current_period_end: i64
}
impl NewSubscriptionDto {
    pub fn into_domain(self) -> Result<Subscription> {
        if self.user_id.is_none() {
            return Err(Error::BadRequest("User id is required".to_string()));
        }
        Ok(Subscription::new(
            self.user_id.unwrap(),
            self.customer_id,
            self.plan.price_id,
            self.subscription_id,
            self.status,
            DateTime::<Utc>::from_timestamp(self.current_period_end, 0)
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