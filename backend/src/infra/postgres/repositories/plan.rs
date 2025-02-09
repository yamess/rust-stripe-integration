use std::sync::Arc;
use diesel::{RunQueryDsl, SelectableHelper};
use crate::domain::plans::entities::Plan;
use crate::domain::plans::repository::PlanRepository;
use crate::infra::postgres::connection::{get_connection, DbPool};
use crate::infra::postgres::models::plan::{NewPlanModel, PlanModel};
use crate::prelude::*;


#[derive(Clone)]
pub struct PostgresPlanRepository {
    pool: Arc<DbPool>,
}
impl PostgresPlanRepository {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

impl PlanRepository for PostgresPlanRepository {
    async fn create(&self, plan: &Plan) -> Result<Plan> {
        let mut connection = get_connection(self.pool.clone())?;
        let new_plan_model = NewPlanModel::try_from(plan)?;
        let plan_model = diesel::insert_into(crate::schema::plans::table)
            .values(&new_plan_model)
            .returning(PlanModel::as_select())
            .get_result(&mut connection).map_err(|e| Error::Database(e.to_string()))?;
        let plan = Plan::try_from(plan_model)?;
        Ok(plan)
    }

    async fn get(&self, id: i32) -> Result<Plan> {
        todo!()
    }

    async fn list(&self, skip: i64, limit: i64) -> Result<Vec<Plan>> {
        todo!()
    }

    async fn update(&self, plan: &Plan) -> Result<Plan> {
        todo!()
    }

    async fn delete(&self, plan_id: i32) -> Result<()> {
        todo!()
    }
}