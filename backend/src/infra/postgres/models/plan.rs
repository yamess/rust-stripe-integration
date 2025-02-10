use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use crate::domain::plans::entities::plan::Plan;
use crate::schema::plans;
use crate::prelude::*;


#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = plans, check_for_backend(diesel::pg::Pg))]
pub struct PlanModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub stripe_product_id: String,
}
impl TryFrom<PlanModel> for Plan {
    type Error = Error;

    fn try_from(model: PlanModel) -> Result<Self> {
        Ok(Plan::construct(
            model.id,
            model.name,
            model.description,
            model.stripe_product_id,
            vec![]
        ))
    }
}



#[derive(Debug, Insertable)]
#[diesel(table_name = plans, check_for_backend(diesel::pg::Pg))]
pub struct CreatePlanModel {
    pub name: String,
    pub description: Option<String>,
    pub stripe_product_id: String,
}
impl TryFrom<&Plan> for CreatePlanModel {
    type Error = Error;

    fn try_from(plan: &Plan) -> Result<Self> {
        Ok(Self {
            name: plan.name().to_string(),
            description: plan.description().map(|s| s.to_string()),
            stripe_product_id: plan.stripe_product_id().to_string(),
        })
    }
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = plans, check_for_backend(diesel::pg::Pg))]
pub struct UpdatePlanModel {
    pub name: String,
    pub description: Option<String>,
}
impl TryFrom<&Plan> for UpdatePlanModel {
    type Error = Error;

    fn try_from(plan: &Plan) -> Result<Self> {
        Ok(Self {
            name: plan.name().to_string(),
            description: plan.description().map(|s| s.to_string()),
        })
    }
}