use std::sync::Arc;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use crate::domain::plans::entities::rate::Rate;
use crate::domain::plans::repository::RateRepository;
use crate::infra::postgres::connection::{get_connection, DbPool};
use crate::infra::postgres::models::rate::{CreateRateModel, RateModel, UpdateRateModel};
use crate::prelude::*;
use diesel::ExpressionMethods;


#[derive(Clone)]
pub struct PostgresRateRepository {
    pool: Arc<DbPool>,
}
impl PostgresRateRepository {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}
impl RateRepository for PostgresRateRepository {
    async fn create(&self, rate: &Rate) -> Result<Rate> {
        let new_rate_model = CreateRateModel::try_from(rate)?;
        let mut connection = get_connection(self.pool.clone())?;
        let rate_model = diesel::insert_into(crate::schema::rates::table)
            .values(&new_rate_model)
            .returning(RateModel::as_select())
            .get_result(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;
        let rate = Rate::try_from(rate_model)?;
        Ok(rate)
    }

    async fn get(&self, id: i32) -> Result<Rate> {
        let mut connection = get_connection(self.pool.clone())?;
        let rate_model = crate::schema::rates::table
            .filter(crate::schema::rates::id.eq(id))
            .first::<crate::infra::postgres::models::rate::RateModel>(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;
        let rate = Rate::try_from(rate_model)?;
        Ok(rate)
    }

    async fn get_by_stripe_price_id(&self, stripe_price_id: &str) -> Result<Rate> {
        let mut connection = get_connection(self.pool.clone())?;
        let rate_model = crate::schema::rates::table
            .filter(crate::schema::rates::stripe_price_id.eq(stripe_price_id))
            .first::<crate::infra::postgres::models::rate::RateModel>(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;
        let rate = Rate::try_from(rate_model)?;
        Ok(rate)
    }

    async fn list(&self, skip: i64, limit: i64) -> Result<Vec<Rate>> {
        let mut connection = get_connection(self.pool.clone())?;
        let rate_models = crate::schema::rates::table
            .offset(skip)
            .limit(limit)
            .load::<crate::infra::postgres::models::rate::RateModel>(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;
        let rates = rate_models.into_iter().map(Rate::try_from).collect::<Result<Vec<Rate>>>()?;
        Ok(rates)
    }

    async fn update(&self, rate: &Rate) -> Result<Rate> {
        let update_rate_model = UpdateRateModel::try_from(rate)?;
        let mut connection = get_connection(self.pool.clone())?;
        let rate_model = diesel::update(crate::schema::rates::table)
            .filter(crate::schema::rates::id.eq(rate.id()))
            .set(&update_rate_model)
            .get_result::<RateModel>(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;
        let rate = Rate::try_from(rate_model)?;
        Ok(rate)
    }

    async fn delete(&self, id: i32) -> Result<()> {
        let mut connection = get_connection(self.pool.clone())?;
        diesel::delete(crate::schema::rates::table)
            .filter(crate::schema::rates::id.eq(id))
            .execute(&mut connection)
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

}