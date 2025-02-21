use crate::application::user::dtos::UpdateUserDto;
use crate::application::user::service::UserService;
use crate::domain::payment::entities::customer::Customer;
use crate::domain::user::repositories::UserRepository;
use crate::prelude::*;

#[derive(Clone)]
pub struct UpdateUserEvent<U: UserRepository> {
    service: UserService<U>,
}
impl<U: UserRepository> UpdateUserEvent<U> {
    pub fn new(service: UserService<U>) -> Self {
        Self { service }
    }

    pub async fn execute(&self, customer: Customer) -> Result<()> {
        let user = self.service.get_by_email(&customer.email()).await?;

        let updates = UpdateUserDto::new(
            Some(customer.id()),
            Some(user.status()),
            user.profile().first_name().map(|s| s.to_string()),
            user.profile().last_name().map(|s| s.to_string()),
            user.profile().phone().map(|s| s.to_string()),
            user.profile().photo_url().map(|s| s.to_string()),
        );
        let _ = self.service.update(updates, &user).await?;
        Ok(())
    }
}
