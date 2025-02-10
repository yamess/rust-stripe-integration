use std::sync::Arc;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, ExpressionMethods};
use crate::domain::plans::entities::plan::Plan;
use crate::domain::plans::repository::PlanRepository;
use crate::infra::postgres::connection::{get_connection, DbPool};
use crate::infra::postgres::models::plan::{CreatePlanModel, PlanModel, UpdatePlanModel};
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
        let new_plan_model = CreatePlanModel::try_from(plan)?;

        let mut connection = get_connection(self.pool.clone())?;
        let plan_model = diesel::insert_into(crate::schema::plans::table)
            .values(&new_plan_model)
            .returning(PlanModel::as_select())
            .get_result(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;
        let plan = Plan::try_from(plan_model)?;
        Ok(plan)
    }

    async fn get(&self, id: i32) -> Result<Plan> {
        let mut connection = get_connection(self.pool.clone())?;
        let plan_model = crate::schema::plans::table
            .filter(crate::schema::plans::id.eq(id))
            .first::<PlanModel>(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;
        let plan = Plan::try_from(plan_model)?;
        Ok(plan)
    }
    async fn get_by_stripe_product_id(&self, stripe_product_id: &str) -> Result<Plan> {
        let mut connection = get_connection(self.pool.clone())?;
        let plan_model = crate::schema::plans::table
            .filter(crate::schema::plans::stripe_product_id.eq(stripe_product_id))
            .first::<PlanModel>(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;
        let plan = Plan::try_from(plan_model)?;
        Ok(plan)
    }

    async fn list(&self, skip: i64, limit: i64) -> Result<Vec<Plan>> {
        let mut connection = get_connection(self.pool.clone())?;
        let plan_models = crate::schema::plans::table
            .offset(skip)
            .limit(limit)
            .load::<PlanModel>(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;
        let plans = plan_models.into_iter().map(Plan::try_from).collect::<Result<Vec<Plan>>>()?;
        Ok(plans)
    }

    async fn update(&self, plan: &Plan) -> Result<Plan> {
        let update_plan_model = UpdatePlanModel::try_from(plan)?;
        let mut connection = get_connection(self.pool.clone())?;
        let plan_model = diesel::update(crate::schema::plans::table)
            .filter(crate::schema::plans::id.eq(plan.id()))
            .set(&update_plan_model)
            .get_result::<PlanModel>(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;
        let plan = Plan::try_from(plan_model)?;
        Ok(plan)
    }

    async fn delete(&self, id: i32) -> Result<()> {
        let mut connection = get_connection(self.pool.clone())?;
        diesel::delete(crate::schema::plans::table)
            .filter(crate::schema::plans::id.eq(id))
            .execute(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }
}