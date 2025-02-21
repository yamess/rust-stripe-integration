use crate::domain::user::entities::User;
use crate::prelude::*;
use uuid::Uuid;

pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<User>;
    async fn find(&self, user_id: &Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_firebase_id(&self, firebase_id: &str) -> Result<Option<User>>;
    async fn find_by_strip_customer_id(&self, strip_customer_id: &str) -> Result<Option<User>>;
    async fn update(&self, user: &User) -> Result<User>;
    async fn delete(&self, user_id: &Uuid) -> Result<()>;
}
