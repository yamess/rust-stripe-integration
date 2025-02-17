use crate::application::subscription::dtos::NewSubscriptionDto;
use crate::application::subscription::service::SubscriptionService;
use crate::application::user::service::UserService;
use crate::domain::subscription::repository::SubscriptionRepository;
use crate::domain::user::repositories::UserRepository;
use crate::prelude::*;


pub struct NewSubscriptionCommand<S, U> {
    subscription_service: SubscriptionService<S>,
    user_service: UserService<U>
}
impl<S: SubscriptionRepository, U: UserRepository> NewSubscriptionCommand<S, U> {
    pub fn new(subscription_service: SubscriptionService<S>, user_service: UserService<U>) -> Self {
        Self { subscription_service, user_service }
    }

    pub async fn execute(&self, data: NewSubscriptionDto) -> Result<()> {
        let user = self.user_service.get_by_payment_provider_id(&data.customer_id).await?;
        let mut data = data;
        data.user_id = Some(user.id());
        self.subscription_service.create(data).await?;
        Ok(())
    }
}