use std::sync::Arc;
use crate::domain::plans::entities::plan::Plan;
use crate::domain::plans::repository::PlanRepository;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PlanService<R> {
    repo: Arc<R>,
}

impl<R: PlanRepository> PlanService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self {
            repo,
        }
    }

    pub async fn create_plan(&self, plan: &Plan) -> Result<Plan> {
        self.repo.create(plan).await
    }
}