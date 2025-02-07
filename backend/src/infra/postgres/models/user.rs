use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use diesel;
use uuid::Uuid;
use crate::prelude::*;
use crate::domain::user::entities::{Profile, User};
use crate::domain::user::value_objects::role::Role;
use crate::domain::user::value_objects::user_status::UserStatus;
use crate::infra::postgres::models::profile::ProfileModel;
use crate::schema;


#[derive(Debug, Insertable)]
#[diesel(table_name = schema::users, check_for_backend(diesel::pg::Pg))]
pub struct CreateUserModel {
    email: String,
    firebase_id: String,
    stripe_customer_id: String,
    status: String,
    role: String,
}
impl TryFrom<&User> for CreateUserModel {
    type Error = Error;

    fn try_from(user: &User) -> Result<Self> {
        Ok(Self {
            email: user.email().to_string(),
            firebase_id: user.firebase_id().to_string(),
            stripe_customer_id: user.stripe_customer_id().to_string(),
            status: user.status().to_string(),
            role: user.role().to_string(),
        })
    }
}


#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::users)]
pub struct UserModel {
    id: Uuid,
    email: String,
    firebase_id: String,
    stripe_customer_id: String,
    status: String,
    role: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<(UserModel, ProfileModel)> for User {
    type Error = Error;

    fn try_from(model: (UserModel, ProfileModel)) -> Result<Self> {
        let user = model.0;
        let profile = Profile::try_from(model.1)?;

        Ok(User::new(
            user.id,
            user.email,
            user.firebase_id,
            user.stripe_customer_id,
            UserStatus::try_from(user.status)?,
            Role::try_from(user.role)?,
            user.created_at,
            user.updated_at,
            profile,
        ))
    }
}
