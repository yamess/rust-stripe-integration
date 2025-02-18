use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::subscription::value_objects::subscription_status::SubscriptionStatus;

#[derive(Debug, Clone)]
pub struct Subscription {
    id: i32,
    user_id: Uuid,
    stripe_customer_id: String,
    stripe_price_id: String,
    stripe_subscription_id: String,
    status: SubscriptionStatus,
    current_period_end: Option<DateTime<Utc>>,
    canceled_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}
impl Subscription {
    pub fn new(
        user_id: Uuid,
        stripe_customer_id: String,
        stripe_price_id: String,
        stripe_subscription_id: String,
        status: SubscriptionStatus,
        current_period_end: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: Default::default(),
            user_id,
            stripe_customer_id,
            stripe_price_id,
            stripe_subscription_id,
            status,
            current_period_end,
            canceled_at: None,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn stripe_customer_id(&self) -> &str {
        &self.stripe_customer_id
    }

    pub fn stripe_price_id(&self) -> &str {
        &self.stripe_price_id
    }

    pub fn stripe_subscription_id(&self) -> &str {
        &self.stripe_subscription_id
    }

    pub fn status(&self) -> &SubscriptionStatus {
        &self.status
    }

    pub fn current_period_end(&self) -> Option<DateTime<Utc>> {
        self.current_period_end
    }

    pub fn canceled_at(&self) -> Option<DateTime<Utc>> {
        self.canceled_at
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        self.updated_at
    }

    pub fn update(
        &mut self,
        stripe_price_id: Option<String>,
        stripe_subscription_id: Option<String>,
        status: Option<SubscriptionStatus>,
        current_period_end: Option<DateTime<Utc>>,
        canceled_at: Option<DateTime<Utc>>,
    ) {
        if let Some(status) = status {
            self.status = status;
        }
        if let Some(stripe_price_id) = stripe_price_id {
            self.stripe_price_id = stripe_price_id;
        }
        if let Some(stripe_subscription_id) = stripe_subscription_id {
            self.stripe_subscription_id = stripe_subscription_id;
        }
        if let Some(current_period_end) = current_period_end {
            self.current_period_end = Some(current_period_end);
        }
        if let Some(canceled_at) = canceled_at {
            self.canceled_at = Some(canceled_at);
        }
        self.updated_at = Some(Utc::now());
    }

    pub fn is_active(&self) -> bool {
        self.status == SubscriptionStatus::Active
    }

    pub fn is_canceled(&self) -> bool {
        self.status == SubscriptionStatus::Canceled
    }

    pub fn construct(
        id: i32,
        user_id: Uuid,
        stripe_customer_id: String,
        stripe_price_id: String,
        stripe_subscription_id: String,
        status: SubscriptionStatus,
        current_period_end: Option<DateTime<Utc>>,
        canceled_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            user_id,
            stripe_customer_id,
            stripe_price_id,
            stripe_subscription_id,
            status,
            current_period_end,
            canceled_at,
            created_at,
            updated_at,
        }
    }
}
