use crate::application::user::dtos::{NewUserDto, UserDto};
use crate::application::user::service::{AuthenticationService, UserService};
use crate::domain::user::entities::{AuthProviderData, User};
use crate::domain::user::repositories::UserRepository;
use crate::domain::user::services::Authenticator;
use crate::prelude::*;


#[derive(Clone)]
pub struct RegisterNewUserUseCase<U: UserRepository, A: Authenticator> {
    user_service: UserService<U>,
    auth_service: AuthenticationService<A>,
}
impl<U: UserRepository, A: Authenticator> RegisterNewUserUseCase<U, A> {
    pub fn new(user_service: UserService<U>, auth_service: AuthenticationService<A>) -> Self {
        Self {
            user_service,
            auth_service,
        }
    }

    pub async fn execute(&self, auth: AuthProviderData) -> Result<UserDto> {
        if auth.email != new_user.email {
            return Err(Error::BadRequest("Email does not match".to_string()));
        }
        let user = User::new(new_user.email, auth.id, "".to_string());
        Ok(user)
    }
}