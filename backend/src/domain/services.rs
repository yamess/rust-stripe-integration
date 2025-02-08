use uuid::Uuid;
use crate::domain::billing::entities::plan::Plan;
use crate::domain::user::entities::User;
use crate::prelude::*;

pub trait PaymentProviderService: Send + Sync {
    async fn create_customer(&self, user: &User) -> Result<serde_json::Value>;
    async fn create_price(&self, product_id: &str, plan: &Plan) -> Result<serde_json::Value>;
    // async fn get_plans(&self) -> Result<Vec<Plan>>;
    // async fn create_checkout_session(&self, user_id: Uuid, plan_id: i32) -> Result<String>;
    // async fn upgrade_subscription(&self, user_id: Uuid, plan_id: i32) -> Result<()>;
    // async fn cancel_subscription(&self, user_id: Uuid) -> Result<()>;
}