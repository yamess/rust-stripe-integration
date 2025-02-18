use std::sync::Arc;
use uuid::Uuid;
use crate::application::subscription::dtos::NewSubscriptionDto;
use crate::domain::subscription::entities::Subscription;
use crate::domain::subscription::repository::SubscriptionRepository;
use crate::prelude::*;



#[derive(Clone)]
pub struct SubscriptionService<S> {
    repo: Arc<S>
}
impl<C: SubscriptionRepository> SubscriptionService<C> {
    pub fn new(repo: Arc<C>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, new_subscription: NewSubscriptionDto) -> Result<Subscription> {
        let subscription = new_subscription.into_domain()?;
        self.repo.save(&subscription).await
    }
    pub async fn find(&self, id: i32) -> Result<Subscription> {
        self.repo.find(id).await
    }
    pub async fn find_by_stripe_subscription_id(&self, subscription_id: &str) -> Result<Subscription> {
        self.repo.find_by_strip_subscription_id(subscription_id).await
    }
    pub async fn find_customer_id(&self, customer_id: &str) -> Result<Subscription> {
        self.repo.find_by_customer_id(customer_id).await
    }
    pub async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Subscription> {
        self.repo.find_by_user_id(user_id).await
    }
    pub async fn update(&self, updates: &Subscription) -> Result<Subscription> {
        self.repo.update(updates).await
    }
}
