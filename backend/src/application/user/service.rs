use std::sync::Arc;
use crate::domain::user::repositories::UserRepository;


#[derive(Debug, Clone)]
pub struct UserService<A: UserRepository> {
    pub user_repo: Arc<A>,
}
impl<A: UserRepository> UserService<A> {
    pub fn new(user_repo: Arc<A>) -> Self {
        Self { user_repo }
    }

    pub fn register(&self) {
        // Create new user
    }

    pub fn get(&self, id: Uuid) {
        // Get user
    }
}