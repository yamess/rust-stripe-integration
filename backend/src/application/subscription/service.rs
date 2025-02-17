use std::sync::Arc;
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

    }
}
