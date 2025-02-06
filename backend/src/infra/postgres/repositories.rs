use std::sync::Arc;
use uuid::Uuid;
use crate::domain::user::entities::User;
use crate::domain::user::repositories::UserRepository;
use crate::infra::postgres::connection::DbPool;
use crate::prelude::*;


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
    async fn save(&self, user: User) -> Result<User> {
        unimplemented!()
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