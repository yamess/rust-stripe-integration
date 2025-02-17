use serde::Deserialize;
use crate::domain::subscription::entities::Subscription;
use crate::schema::users::stripe_customer_id;

#[derive(Debug, Clone, Deserialize)]
pub struct NewSubscriptionDto {
    pub user_id: uuid::Uuid,
    pub stripe_customer_id: String,
    pub payment_method_id: i32,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}
impl NewSubscriptionDto {
    pub fn into_domain(self) -> Subscription {
        Subscription::new(
            self.user_id,
            self.plan_id,
            self.payment_method_id,
            self.start_date,
            self.end_date,
            self.status,
            self.created_at,
            self.updated_at,
        )
    }
}