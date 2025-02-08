use std::sync::Arc;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;
use crate::domain::billing::entities::plan::Plan;
use crate::domain::billing::repository::BillingRepository;
use crate::infra::postgres::connection::{get_connection, DbPool};
use crate::infra::postgres::models::plan::{NewPlanModel, PlanModel};
use crate::prelude::*;
use crate::schema;
use crate::schema::plans::dsl::plans;
use crate::schema::profiles::first_name;

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
    async fn save_plan(&self, plan: &Plan) -> Result<Plan> {
        let mut connection = get_connection(self.pool.clone())?;
        let new_plan_model = NewPlanModel::try_from(plan)?;
        let plan_model = diesel::insert_into(crate::schema::plans::table)
            .values(&new_plan_model)
            .returning(PlanModel::as_select())
            .get_result(&mut connection).map_err(|e| Error::Database(e.to_string()))?;
        let plan = Plan::try_from(plan_model)?;
        Ok(plan)
    }

    async fn get_plan(&self, id: i32) -> Result<Plan> {
        let mut connection = get_connection(self.pool.clone())?;
        let plan_model = plans
            .find(id)
            .select(PlanModel::as_select())
            .first(&mut connection)
            .optional()
            .map_err(|e| Error::Database(e.to_string()))?;
        let plan = match plan_model {
            Some(plan) => Plan::try_from(plan)?,
            None => return Err(Error::NotFound("Plan WITH ID {} not found".to_string())),
        };
        Ok(plan)
    }

    async fn get_plans(&self, skip: i64, limit: i64) -> Result<Vec<Plan>> {
        let mut connection = get_connection(self.pool.clone())?;

        let plan_models = plans
            .select(PlanModel::as_select())
            .offset(skip as i64)
            .limit(limit as i64)
            .load::<PlanModel>(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;

        let vec_plans = plan_models
            .into_iter()
            .map(|plan| Plan::try_from(plan))
            .collect::<Result<Vec<Plan>>>()?;

        Ok(vec_plans)
    }
}