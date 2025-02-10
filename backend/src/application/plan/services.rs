use std::sync::Arc;
use crate::domain::plans::repository::PlanRepository;


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
}