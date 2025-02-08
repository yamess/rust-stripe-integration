use std::sync::Arc;
use crate::domain::billing::repository::BillingRepository;
use crate::prelude::*;

#[derive(Clone)]
pub struct PlanService<P> {
    repo: Arc<P>
}
impl<P: BillingRepository > PlanService<P> {
    pub fn new(service: P) -> Self {
        Self { service }
    }
    pub async fn save_plan(&self, plan: &Plan) -> Result<Plan> {
        self.service.save_plan(plan).await
    }
    pub async fn get_plan(&self, plan_id: i32) -> Result<Plan> {
        self.service.get_plan(plan_id).await
    }
    pub async fn get_plans(&self, skip: i64, limit: i64) -> Result<Vec<Plan>> {
        self.service.get_plans(skip, limit).await
    }
    pub async fn update_plan(&self, plan: &Plan) -> Result<Plan> {
        self.service.update_plan(plan).await
    }
    pub async fn delete_plan(&self, plan_id: i32) -> Result<()> {
        self.service.delete_plan(plan_id).await
    }
}