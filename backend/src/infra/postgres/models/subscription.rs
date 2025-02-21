use crate::domain::subscription::entities::Subscription;
use crate::domain::subscription::value_objects::subscription_status::SubscriptionStatus;
use crate::prelude::*;
use crate::schema;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::subscriptions)]
pub struct CreateSubscriptionModel {
    user_id: Uuid,
    stripe_customer_id: String,
    stripe_price_id: String,
    stripe_product_id: String,
    stripe_subscription_id: String,
    status: String,
    has_used_trial: bool,
    current_period_end: Option<DateTime<Utc>>,
    cancel_at_period_end: bool,
}
impl TryFrom<&Subscription> for CreateSubscriptionModel {
    type Error = Error;

    fn try_from(subscription: &Subscription) -> Result<Self> {
        Ok(Self {
            user_id: *subscription.user_id(),
            stripe_customer_id: subscription.stripe_customer_id().to_string(),
            stripe_price_id: subscription.stripe_price_id().to_string(),
            stripe_product_id: subscription.stripe_product_id().to_string(),
            stripe_subscription_id: subscription.stripe_subscription_id().to_string(),
            status: subscription.status().to_string(),
            has_used_trial: subscription.has_used_trial(),
            current_period_end: subscription.current_period_end(),
            cancel_at_period_end: subscription.cancel_at_period_end(),
        })
    }
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::subscriptions)]
#[diesel(belongs_to(UserModel, foreign_key = user_id))]
pub struct SubscriptionModel {
    pub id: i32,
    pub user_id: Uuid,
    pub stripe_customer_id: String,
    pub stripe_price_id: String,
    pub stripe_product_id: String,
    pub stripe_subscription_id: String,
    pub status: String,
    pub has_used_trial: bool,
    pub current_period_end: Option<DateTime<Utc>>,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl TryFrom<SubscriptionModel> for Subscription {
    type Error = Error;

    fn try_from(model: SubscriptionModel) -> Result<Self> {
        Ok(Subscription::construct(
            model.id,
            model.user_id,
            model.stripe_customer_id,
            model.stripe_price_id,
            model.stripe_product_id,
            model.stripe_subscription_id,
            SubscriptionStatus::from_str(&model.status)?,
            model.has_used_trial,
            model.current_period_end,
            model.cancel_at_period_end,
            model.canceled_at,
            model.created_at,
            model.updated_at,
        ))
    }
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = schema::subscriptions)]
pub struct UpdateSubscriptionModel {
    pub stripe_price_id: String,
    pub stripe_product_id: String,
    pub stripe_subscription_id: String,
    pub status: String,
    pub has_used_trial: bool,
    pub current_period_end: Option<DateTime<Utc>>,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl TryFrom<&Subscription> for UpdateSubscriptionModel {
    type Error = Error;

    fn try_from(subscription: &Subscription) -> Result<Self> {
        Ok(Self {
            stripe_price_id: subscription.stripe_price_id().to_string(),
            stripe_product_id: subscription.stripe_product_id().to_string(),
            stripe_subscription_id: subscription.stripe_subscription_id().to_string(),
            status: subscription.status().to_string(),
            has_used_trial: subscription.has_used_trial(),
            current_period_end: subscription.current_period_end(),
            cancel_at_period_end: subscription.cancel_at_period_end(),
            canceled_at: subscription.canceled_at(),
            updated_at: subscription.updated_at(),
        })
    }
}
