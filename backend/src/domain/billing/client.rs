use uuid::Uuid;
use crate::prelude::*;

pub trait BillingProviderClient: Send + Sync {
    async fn create_customer(&self, email: &str) -> Result<()>;
    async fn create_price(&self, product_id: &str, price: i32, currency: &str) -> Result<()>;
    // async fn create_checkout_session(&self, user_id: &Uuid, plan_id: i32) -> Result<()>;
    // async fn create_subscription(&self, user_id: &Uuid, plan_id: i32) -> Result<()>;
    // async fn cancel_subscription(&self, user_id: &Uuid) -> Result<()>;
    // async fn update_subscription(&self, user_id: &Uuid, plan_id: i32) -> Result<()>;
    // async fn get_subscription(&self, user_id: &Uuid) -> Result<()>;
    // async fn get_plans(&self) -> Result<()>;
    // async fn get_plan(&self, plan_id: i32) -> Result<()>;
    // async fn get_customer(&self, user_id: &Uuid) -> Result<()>;
}