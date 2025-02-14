use uuid::Uuid;
use crate::application::user::dtos::{NewUserDto, UpdateUserDto, UserDto};
use crate::application::user::service::{AuthenticationService, UserService};
use crate::domain::user::entities::{AuthProviderData, User};
use crate::domain::user::repositories::UserRepository;
use crate::domain::user::services::Authenticator;
use crate::prelude::*;


#[derive(Clone)]
pub struct RegisterNewUserUseCase<U: UserRepository> {
    user_service: UserService<U>,
}
impl<U: UserRepository> RegisterNewUserUseCase<U> {
    pub fn new(user_service: UserService<U>) -> Self {
        Self {
            user_service,
        }
    }

    pub async fn execute(&self, auth: &AuthProviderData) -> Result<UserDto> {
        let user = User::new(auth.email.clone(), auth.id.to_string(), None);
        let user = match self.user_service.register(&user).await {
            Ok(user) => user,
            Err(Error::UserAlreadyExists) => {
                tracing::info!("User already exists, fetching user data");
                let user = self.user_service.get_by_auth_provider_id(&auth.id).await?;
                user
            },
            Err(e) => return Err(e)
        };
        Ok(user)
    }
}

#[derive(Clone)]
pub struct GetUserByIdUseCase<U: UserRepository> {
    user_service: UserService<U>,
}
impl<U: UserRepository> GetUserByIdUseCase<U> {
    pub fn new(user_service: UserService<U>) -> Self {
        Self { user_service }
    }

    pub async fn execute(&self, id: &Uuid) -> Result<UserDto> {
        let user = self.user_service.get(id).await?;
        Ok(user)
    }
}

#[derive(Clone)]
pub struct GetUserByAuthProviderIdUseCase<U: UserRepository> {
    user_service: UserService<U>
}
impl<U: UserRepository> GetUserByAuthProviderIdUseCase<U> {
    pub fn new(user_service: UserService<U>) -> Self {
        Self {user_service}
    }

    pub async fn execute(&self, auth_id: &str) -> Result<UserDto> {
        let user = self.user_service.get_by_auth_provider_id(auth_id).await?;
        Ok(user)
    }
}

#[derive(Clone)]
pub struct GetUserByPaymentProviderIdUseCase<U: UserRepository> {
    user_service: UserService<U>
}
impl <U: UserRepository> GetUserByPaymentProviderIdUseCase<U> {
    pub fn new(user_service: UserService<U>) -> Self {
        Self {user_service}
    }

    pub async fn execute(&self, payment_id: &str) -> Result<UserDto> {
        let user = self.user_service.get_by_payment_provider_id(payment_id).await?;
        Ok(user)
    }
}

#[derive(Clone)]
pub struct GetUserByEmailUseCase<U: UserRepository> {
    user_service: UserService<U>
}
impl<U: UserRepository> GetUserByEmailUseCase<U> {
    pub fn new(user_service: UserService<U>) -> Self {
        Self {user_service}
    }

    pub async fn execute(&self, email: &str) -> Result<UserDto> {
        let user = self.user_service.get_by_email(email).await?;
        Ok(user)
    }
}

#[derive(Clone)]
pub struct UpdateUserUseCase<U: UserRepository> {
    user_service: UserService<U>
}
impl<U: UserRepository> UpdateUserUseCase<U> {
    pub fn new(user_service: UserService<U>) -> Self {
        Self {user_service}
    }

    pub async fn execute(&self, updates: UpdateUserDto, user: &UserDto) -> Result<UserDto> {
        let mut user = User::try_from(user)?;
        let user = self.user_service.update(updates, &mut user).await?;
        Ok(user)
    }
}

pub struct DeleteUserUseCase<U: UserRepository> {
    user_service: UserService<U>
}
impl<U: UserRepository> DeleteUserUseCase<U> {
    pub fn new(user_service: UserService<U>) -> Self {
        Self {user_service}
    }

    pub async fn execute(&self, user: &UserDto) -> Result<()> {
        self.user_service.delete(&user.id).await?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct ExtractUserUseCase<U: UserRepository, A: Authenticator> {
    authenticator: AuthenticationService<A>,
    user_service: UserService<U>
}
impl<U: UserRepository, A: Authenticator> ExtractUserUseCase<U, A> {
    pub fn new(authenticator: AuthenticationService<A>, user_service: UserService<U>) -> Self {
        Self {authenticator, user_service}
    }

    pub async fn execute(&self, token: &str) -> Result<UserDto> {
        let auth_data = self.authenticator.authenticate(token).await?;
        let user = self.user_service.get_by_auth_provider_id(&auth_data.id).await?;
        Ok(user)
    }
}

#[derive(Clone)]
pub struct AuthenticateUserUseCase<A: Authenticator> {
    authenticator: AuthenticationService<A>
}
impl<A: Authenticator> AuthenticateUserUseCase<A> {
    pub fn new(authenticator: AuthenticationService<A>) -> Self {
        Self {authenticator}
    }

    pub async fn execute(&self, token: &str) -> Result<AuthProviderData> {
        let auth_data = self.authenticator.authenticate(token).await?;
        Ok(auth_data)
    }
}