use uuid::Uuid;
use crate::domain::payment::entities::PaymentSession;
use crate::domain::plans::entities::{Plan, RatePlan};
use crate::domain::user::entities::User;
use crate::prelude::*;

pub trait PaymentService: Send + Sync {
    async fn create_customer(&self, email: &str) -> Result<serde_json::Value>;
    async fn create_product(&self, name: &str) -> Result<serde_json::Value>;
    async fn create_price(&self, product_id: &str, rate: &RatePlan) -> Result<serde_json::Value>;
    async fn create_checkout_session(&self, user: &User, plan: &Plan) -> Result<PaymentSession>;
    // async fn get_plans(&self) -> Result<Vec<Plan>>;
    // async fn create_checkout_session(&self, user_id: Uuid, plan_id: i32) -> Result<String>;
    // async fn upgrade_subscription(&self, user_id: Uuid, plan_id: i32) -> Result<()>;
    // async fn cancel_subscription(&self, user_id: Uuid) -> Result<()>;
}