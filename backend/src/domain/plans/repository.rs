use crate::prelude::*;


pub trait PlanRepository: Send + Sync {
    async fn create(&self, plan: &Plan) -> Result<Plan>;
    async fn get(&self, id: i32) -> Result<Plan>;
    async fn list(&self, skip: i64, limit: i64) -> Result<Vec<Plan>>;
    async fn update(&self, plan: &Plan) -> Result<Plan>;
    async fn delete(&self, plan_id: i32) -> Result<()>;
}