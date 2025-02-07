use std::sync::Arc;
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::row::NamedRow;
use uuid::Uuid;
use crate::domain::user::entities::User;
use crate::domain::user::repositories::UserRepository;
use crate::infra::postgres::connection::{get_connection, DbPool};
use crate::infra::postgres::models::profile::{CreateProfileModel, ProfileModel};
use crate::infra::postgres::models::user::{CreateUserModel, UserModel};
use crate::prelude::*;
use crate::schema::users::dsl::users;
use crate::schema::profiles::dsl::profiles;
use crate::schema;
use diesel::ExpressionMethods;


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
        let mut new_profile = CreateProfileModel::try_from(user.profile())?;

        let response = connection.build_transaction().run::<_, diesel::result::Error, _>(|conn| {
            let user = diesel::insert_into(users)
                .values(&new_user)
                .returning(UserModel::as_select())
                .get_result(conn)?;
            new_profile.user_id = user.id;
            let profile = diesel::insert_into(profiles)
                .values(&new_profile)
                .returning(ProfileModel::as_select())
                .get_result(conn)?;
            Ok((user, profile))
        }).map_err(|e| Error::Database(e.to_string()))?;

        let user = User::try_from(response)?;
        Ok(user)
    }

    async fn find(&self, user_id: &Uuid) -> Result<Option<User>> {
        let mut connection = get_connection(self.pool.clone())?;
        let user = users
            .inner_join(profiles)
            .filter(schema::users::id.eq(user_id))
            .get_result::<(UserModel, ProfileModel)>(&mut connection)
            .optional()
            .map_err(|e| Error::Database(e.to_string()))?;

        let result = user.map(|(user, profile)| {
            let model =
            User::try_from((user, profile));
            model
        }).transpose()?;
        Ok(result)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let mut connection = get_connection(self.pool.clone())?;
        let user = users
            .inner_join(profiles)
            .filter(schema::users::email.eq(email))
            .get_result::<(UserModel, ProfileModel)>(&mut connection)
            .optional()
            .map_err(|e| Error::Database(e.to_string()))?;

        let result = user.map(|(user, profile)| {
            let model =
            User::try_from((user, profile));
            model
        }).transpose()?;
        Ok(result)
    }

    async fn find_by_firebase_id(&self, firebase_id: &str) -> Result<Option<User>> {
        let mut connection = get_connection(self.pool.clone())?;
        let user = users
            .inner_join(profiles)
            .filter(schema::users::firebase_id.eq(firebase_id))
            .get_result::<(UserModel, ProfileModel)>(&mut connection)
            .optional()
            .map_err(|e| Error::Database(e.to_string()))?;

        let result = user.map(|(user, profile)| {
            let model =
            User::try_from((user, profile));
            model
        }).transpose()?;
        Ok(result)
    }

    async fn find_by_strip_customer_id(&self, strip_customer_id: &str) -> Result<Option<User>> {
        let mut connection = get_connection(self.pool.clone())?;
        let user = users
            .inner_join(profiles)
            .filter(schema::users::stripe_customer_id.eq(strip_customer_id))
            .get_result::<(UserModel, ProfileModel)>(&mut connection)
            .optional()
            .map_err(|e| Error::Database(e.to_string()))?;

        let result = user.map(|(user, profile)| {
            let model =
            User::try_from((user, profile));
            model
        }).transpose()?;
        Ok(result)
    }

    async fn update(&self, user: &User) -> Result<User> {
        unimplemented!()
    }

    async fn delete(&self, user_id: &Uuid) -> Result<()> {
        unimplemented!()
    }

}