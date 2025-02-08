use std::sync::Arc;
use crate::domain::billing::entities::plan::Plan;
use crate::domain::billing::repository::BillingRepository;
use crate::infra::postgres::connection::DbPool;

#[derive(Clone)]
pub struct PostgresBillingRepository {
    pool: Arc<DbPool>,
}
impl PostgresBillingRepository {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

impl BillingRepository for PostgresBillingRepository {
    async fn save_plan(&self, plan: &Plan) -> crate::prelude::Result<Plan> {
        todo!()
    }
}