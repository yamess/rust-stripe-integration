use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::application::subscription::dtos::{InvoicePaidEvent, NewSubscriptionDto, PlanObject};
use crate::application::subscription::service::SubscriptionService;
use crate::application::user::service::UserService;
use crate::domain::subscription::repository::SubscriptionRepository;
use crate::domain::subscription::value_objects::subscription_status::SubscriptionStatus;
use crate::domain::user::repositories::UserRepository;
use crate::prelude::*;
use crate::shared::extractors::{extract_string, extract_timestamp};

pub struct InvoicePaidUseCase<S, U> {
    pub subscription_service: SubscriptionService<S>,
    pub user_service: UserService<U>
}
impl<S: SubscriptionRepository, U: UserRepository> InvoicePaidUseCase<S, U> {
    pub fn new(subscription_service: SubscriptionService<S>, user_service: UserService<U>) -> Self {
        Self { subscription_service, user_service }
    }
    pub async fn execute(&self, data: Value,) -> Result<()> {
        let line_data = data["lines"]["data"][0].clone();

        let customer_id = extract_string(&data, "customer")?;
        let subscription_id = extract_string(&data, "subscription")?;
        let current_period_end = extract_timestamp(&line_data, "period/end")?;
        let price_id = extract_string(&line_data, "price/id")?;
        let product_id = extract_string(&line_data, "price/product")?;

        let user = self.user_service.get_by_payment_provider_id(&customer_id).await?;
        let subscription = self.subscription_service.find_by_user_id(&user.id()).await;
        match subscription {
            Ok(mut subscription) => {
                subscription.update(
                    Some(price_id),
                    Some(subscription_id),
                    Some(SubscriptionStatus::Active),
                    Some(current_period_end),
                    None
                );
                self.subscription_service.update(&subscription).await?;
                Ok(())
            },
            Err(Error::NotFound(_)) => {
                let new_subscription = NewSubscriptionDto{
                    user_id: Some(user.id()),
                    subscription_id,
                    customer_id,
                    plan: PlanObject{
                        price_id,
                        product_id: product_id.to_string()
                    },
                    status: SubscriptionStatus::Active,
                    current_period_end: current_period_end.timestamp()
                };
                self.subscription_service.create(new_subscription).await?;
                Ok(())
            }
            Err(e) => Err(e)
        }
    }
}

pub struct InvoicePaymentFailedUseCase<S, U>{
    pub subscription_service: SubscriptionService<S>,
    pub user_service: UserService<U>
}
impl<S: SubscriptionRepository, U: UserRepository> InvoicePaymentFailedUseCase<S, U> {
    pub fn new(subscription_service: SubscriptionService<S>, user_service: UserService<U>) -> Self {
        Self { subscription_service, user_service }
    }
    pub async fn execute(&self, data: Value) -> Result<()> {
        let customer_id = extract_string(&data, "customer")?;
        let user = self.user_service.get_by_payment_provider_id(&customer_id).await?;
        let mut subscription = self.subscription_service.find_by_user_id(&user.id()).await?;
        subscription.update(
            None,
            None,
            Some(SubscriptionStatus::PastDue),
            None,
            None
        );
        self.subscription_service.update(&subscription).await?;
        Ok(())
    }
}

pub struct SubscriptionUpdatedUseCase<S, U>{
    pub subscription_service: SubscriptionService<S>,
    pub user_service: UserService<U>
}
impl<S: SubscriptionRepository, U: UserRepository> SubscriptionUpdatedUseCase<S, U> {
    pub fn new(subscription_service: SubscriptionService<S>, user_service: UserService<U>) -> Self {
        Self { subscription_service, user_service }
    }
    pub async fn execute(&self, data: Value) -> Result<()> {
        let line_data = data["lines"]["data"][0].clone();
        let customer_id = extract_string(&data, "customer")?;
        let subscription_id = extract_string(&data, "subscription")?;
        let price_id = extract_string(&line_data, "price/id")?;
        let product_id = extract_string(&line_data, "price/product")?;

        let user = self.user_service.get_by_payment_provider_id(&customer_id).await?;
        let mut subscription = self.subscription_service.find_by_user_id(&user.id()).await?;
        let status = match extract_string(&data, "status")?.as_str() {
            "active" | "trialing" => SubscriptionStatus::Active,
            "past_due" | "unpaid" => SubscriptionStatus::PastDue,
            "canceled" | "incomplete" | "incomplete_expired" => SubscriptionStatus::Canceled,
            _ => SubscriptionStatus::Unknown
        };
        subscription.update(
            Some(price_id),
            Some(subscription_id),
            Some(status),
            None,
            None
        );
        self.subscription_service.update(&subscription).await?;
        Ok(())
    }
}

pub struct SubscriptionCanceledUseCase<S, U> {
    pub subscription_service: SubscriptionService<S>,
    pub user_service: UserService<U>
}
impl<S: SubscriptionRepository, U: UserRepository> SubscriptionCanceledUseCase<S, U> {
    pub fn new(subscription_service: SubscriptionService<S>, user_service: UserService<U>) -> Self {
        Self { subscription_service, user_service }
    }
    pub async fn execute(&self, data: Value) -> Result<()> {
        let customer_id = extract_string(&data, "customer")?;
        let user = self.user_service.get_by_payment_provider_id(&customer_id).await?;
        let mut subscription = self.subscription_service.find_by_user_id(&user.id()).await?;
        subscription.update(
            None,
            None,
            Some(SubscriptionStatus::Canceled),
            None,
            Some(Utc::now())
        );
        self.subscription_service.update(&subscription).await?;
        Ok(())
    }
}