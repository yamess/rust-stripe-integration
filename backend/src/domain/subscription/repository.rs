use uuid::Uuid;
use crate::prelude::*;
use crate::domain::subscription::entities::Subscription;


pub trait SubscriptionRepository: Send + Sync {
    async fn save(&self, subscription: &Subscription) -> Result<Subscription>;
    async fn find(&self, id: i32) -> Result<Subscription>;
    async fn find_by_strip_subscription_id(&self, subscription_id: &str) -> Result<Subscription>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Subscription>;
    async fn find_by_customer_id(&self, customer_id: &str) -> Result<Subscription>;
    async fn update(&self, subscription: &Subscription) -> Result<Subscription>;
    async fn delete(&self, id: i32) -> Result<()>;
}