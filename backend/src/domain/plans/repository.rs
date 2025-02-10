use crate::domain::plans::entities::plan::Plan;
use crate::domain::plans::entities::rate::Rate;
use crate::prelude::*;

pub trait PlanRepository: Send + Sync {
    async fn create(&self, plan: &Plan) -> Result<Plan>;
    async fn get(&self, id: i32) -> Result<Plan>;
    async fn get_by_stripe_product_id(&self, stripe_product_id: &str) -> Result<Plan>;
    async fn list(&self, skip: i64, limit: i64) -> Result<Vec<Plan>>;
    async fn update(&self, plan: &Plan) -> Result<Plan>;
    async fn delete(&self, plan_id: i32) -> Result<()>;
}

pub trait RateRepository: Send + Sync {
    async fn create(&self, rate: &Rate) -> Result<Rate>;
    async fn get(&self, id: i32) -> Result<Rate>;
    async fn get_by_stripe_price_id(&self, stripe_price_id: &str) -> Result<Rate>;
    async fn list(&self, skip: i64, limit: i64) -> Result<Vec<Rate>>;
    async fn update(&self, rate: &Rate) -> Result<Rate>;
    async fn delete(&self, rate_id: i32) -> Result<()>;
}