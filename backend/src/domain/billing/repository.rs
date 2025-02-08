use uuid::Uuid;
use crate::domain::billing::entities::plan::Plan;
use crate::domain::billing::entities::subscription::Subscription;
use crate::prelude::*;

pub trait BillingRepository: Send + Sync {
    // Plan operations
    async fn save_plan(&self, plan: &Plan) -> Result<Plan>;
    // async fn get_user_plan(&self, user_id: &Uuid) -> Result<Plan>;
    // async fn get_plan(&self, plan_id: i32) -> Result<Plan>;
    // async fn get_plans(&self) -> Result<Vec<Plan>>;
    // async fn update_plan(&self, plan: &Plan) -> Result<Plan>;
    // async fn delete_plan(&self, plan_id: i32) -> Result<()>;
    //
    // // Subscription operations
    // async fn save_subscription(&self, subscription: &Subscription) -> Result<Subscription>;
    // async fn get_user_subscription(&self, user_id: &Uuid) -> Result<Subscription>;
    // async fn get_subscription(&self, subscription_id: i32) -> Result<Subscription>;
    // async fn get_subscriptions(&self) -> Result<Vec<Subscription>>;
    // async fn update_subscription(&self, subscription: &Subscription) -> Result<Subscription>;
    // async fn delete_subscription(&self, subscription_id: i32) -> Result<()>;
}