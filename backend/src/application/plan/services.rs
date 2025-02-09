use std::sync::Arc;
use crate::application::plan::dtos::NewPlanDto;
use crate::domain::payment::service::PaymentService;
use crate::domain::plans::entities::Plan;
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
}