use std::sync::Arc;
use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl, QueryDsl, SelectableHelper};
use uuid::Uuid;
use crate::domain::subscription::entities::Subscription;
use crate::domain::subscription::repository::SubscriptionRepository;
use crate::infra::postgres::connection::{get_connection, DbPool};
use crate::infra::postgres::models::subscription::{CreateSubscriptionModel, SubscriptionModel, UpdateSubscriptionModel};
use crate::schema::subscriptions::dsl::subscriptions;
use crate::prelude::*;
use crate::schema;

#[derive(Clone)]
pub struct PostgresSubscriptionRepository {
    pool: Arc<DbPool>,
}
impl PostgresSubscriptionRepository {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}
impl SubscriptionRepository for PostgresSubscriptionRepository {
    async fn save(&self, subscription: &Subscription) -> Result<Subscription> {
        let model = CreateSubscriptionModel::try_from(subscription)?;
        let mut connection = get_connection(self.pool.clone())?;

        let model = diesel::insert_into(subscriptions)
            .values(&model)
            .get_result::<SubscriptionModel>(&mut connection)
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                    Error::RecordAlreadyExists
                },
                other => Error::Database(other.to_string()),
            })?;

        let subscription = Subscription::try_from(model)?;
        Ok(subscription)
    }
    async fn find(&self, id: i32) -> Result<Subscription> {
        let mut connection = get_connection(self.pool.clone())?;

        let model = subscriptions
            .filter(schema::subscriptions::id.eq(id))
            .get_result::<SubscriptionModel>(&mut connection)
            .optional()
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    let msg = format!("Subscription {} not found", id);
                    Error::NotFound(msg)
                },
                other => Error::Database(other.to_string()),
            })?;

        match model {
            Some(model) => {
                let subscription = Subscription::try_from(model)?;
                Ok(subscription)
            },
            None => Err(Error::NotFound("Subscription {} not found".to_string())),
        }
    }
    async fn find_by_strip_subscription_id(&self, subscription_id: &str) -> Result<Subscription> {
        let mut connection = get_connection(self.pool.clone())?;

        let model = subscriptions
            .filter(schema::subscriptions::stripe_subscription_id.eq(subscription_id))
            .get_result::<SubscriptionModel>(&mut connection)
            .optional()
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    let msg = format!("Subscription with stripe subscription id {} not found", subscription_id);
                    Error::NotFound(msg)
                },
                other => Error::Database(other.to_string()),
            })?;

        match model {
            Some(model) => {
                let subscription = Subscription::try_from(model)?;
                Ok(subscription)
            },
            None => Err(Error::NotFound("Subscription {} not found".to_string())),
        }
    }
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Subscription> {
        let mut connection = get_connection(self.pool.clone())?;

        let models = subscriptions
            .filter(schema::subscriptions::user_id.eq(user_id))
            .get_result::<SubscriptionModel>(&mut connection)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    let msg = format!("Subscription with user id {} not found", user_id);
                    Error::NotFound(msg)
                },
                other => Error::Database(other.to_string()),
            })?;

        let subscription = Subscription::try_from(models)?;
        Ok(subscription)
    }
    async fn find_by_customer_id(&self, customer_id: &str) -> Result<Subscription> {
        let mut connection = get_connection(self.pool.clone())?;

        let model = subscriptions
            .filter(schema::subscriptions::stripe_customer_id.eq(customer_id))
            .get_result::<SubscriptionModel>(&mut connection)
            .optional()
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    let msg = format!("Subscription with stripe customer id {} not found", customer_id);
                    Error::NotFound(msg)
                },
                other => Error::Database(other.to_string()),
            })?;

        match model {
            Some(model) => {
                let subscription = Subscription::try_from(model)?;
                Ok(subscription)
            },
            None => Err(Error::NotFound("Subscription {} not found".to_string())),
        }
    }

    async fn update(&self, subscription: &Subscription) -> Result<Subscription> {
        let model = UpdateSubscriptionModel::try_from(subscription)?;
        let mut connection = get_connection(self.pool.clone())?;

        let model = diesel::update(subscriptions)
            .filter(schema::subscriptions::id.eq(subscription.id()))
            .set(&model)
            .get_result::<SubscriptionModel>(&mut connection)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    let msg = format!("Subscription {} not found", subscription.id());
                    Error::NotFound(msg)
                },
                other => Error::Database(other.to_string()),
            })?;

        let subscription = Subscription::try_from(model)?;
        Ok(subscription)
    }

    async fn delete(&self, id: i32) -> Result<()> {
        let mut connection = get_connection(self.pool.clone())?;

        diesel::delete(subscriptions)
            .filter(schema::subscriptions::id.eq(id))
            .execute(&mut connection)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    let msg = format!("Subscription {} not found", id);
                    Error::NotFound(msg)
                },
                other => Error::Database(other.to_string()),
            })?;

        Ok(())
    }
}
