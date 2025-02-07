use std::sync::Arc;
use diesel::{Connection, RunQueryDsl, SelectableHelper};
use uuid::Uuid;
use crate::domain::user::entities::User;
use crate::domain::user::repositories::UserRepository;
use crate::infra::postgres::connection::{get_connection, DbPool};
use crate::infra::postgres::models::profile::{CreateProfileModel, ProfileModel};
use crate::infra::postgres::models::user::{CreateUserModel, UserModel};
use crate::prelude::*;
use crate::schema::users::dsl::users;
use crate::schema::profiles::dsl::profiles;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: Arc<DbPool>,
}

impl PostgresUserRepository {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: &User) -> Result<User> {
        let mut connection = get_connection(self.pool.clone())?;

        let new_user = CreateUserModel::try_from(user)?;
        let new_profile = CreateProfileModel::try_from(user.profile())?;

        connection.build_transaction().run::<_, diesel::result::Error, _>(|conn| {
            let _ = diesel::insert_into(users)
                .values(&new_user)
                .execute(&conn)?;

            let profile = diesel::insert_into(profiles)
                .values(&new_profile)
                .returning(ProfileModel::as_select())
                .get_result(&conn)?;
                // .get_result::<ProfileModel>(&conn)?;

            let user = users
                .filter(schema::users::email.eq(&user.email()))

                .first::<UserModel>(&conn)?;

            Ok(())
        }).map_err(|e| Error::Database(e.to_string()))?;
    }

    async fn find(&self, user_id: Uuid) -> Result<Option<User>> {
        unimplemented!()
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        unimplemented!()
    }

    async fn find_by_firebase_id(&self, firebase_id: &str) -> Result<Option<User>> {
        unimplemented!()
    }

    async fn find_by_strip_customer_id(&self, strip_customer_id: &str) -> Result<Option<User>> {
        unimplemented!()
    }

    async fn update(&self, user: User) -> Result<User> {
        unimplemented!()
    }

    async fn delete(&self, user_id: Uuid) -> Result<()> {
        unimplemented!()
    }

}