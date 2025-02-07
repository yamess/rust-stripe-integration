use std::sync::Arc;
use crate::domain::user::repositories::UserRepository;
use uuid::Uuid;
use crate::application::user::dtos::{NewUserDto, UpdateUserDto, UserDto};
use crate::domain::user::entities::{AuthProviderData, User};
use crate::domain::user::services::Authenticator;
use crate::prelude::*;


#[derive(Debug, Clone)]
pub struct UserService<U: UserRepository> {
    user_repo: Arc<U>,
}
impl<U: UserRepository> UserService<U> {
    pub fn new(user_repo: Arc<U>) -> Self {
        Self { user_repo }
    }

    pub async fn register(&self, new_user: &User) -> Result<UserDto> {
        let user = self.user_repo.save(&new_user).await?;
        let user = UserDto::try_from(&user)?;
        Ok(user)
    }

    pub async fn get(&self, id: &Uuid) -> Result<UserDto> {
        let user = self.user_repo.find(id).await;
        match user {
            Ok(Some(user)) => {
                let user = UserDto::try_from(&user)?;
                Ok(user)
            }
            Ok(None) => {
                tracing::info!("User {} not found", id);
                Err(Error::NotFound("User not found".to_string()))
            },
            Err(e) => Err(e),
        }
    }

    pub async fn get_by_email(&self, email: &str) -> Result<UserDto> {
        let user = self.user_repo.find_by_email(email).await;
        match user {
            Ok(Some(user)) => {
                let user = UserDto::try_from(&user)?;
                Ok(user)
            }
            Ok(None) => {
                tracing::info!("User with email {} not found", email);
                Err(Error::NotFound("User not found".to_string()))
            },
            Err(e) => Err(e),
        }
    }

    pub async fn get_by_auth_provider_id(&self, auth_id: &str) -> Result<UserDto> {
        let user = self.user_repo.find_by_firebase_id(auth_id).await;
        match user {
            Ok(Some(user)) => {
                let user = UserDto::try_from(&user)?;
                Ok(user)
            }
            Ok(None) => {
                tracing::info!("User with auth id {} not found", auth_id);
                Err(Error::NotFound("User not found".to_string()))
            },
            Err(e) => Err(e),
        }
    }

    pub async fn get_by_payment_provider_id(&self, pay_provider_id: &str) -> Result<UserDto> {
        let user = self.user_repo.find_by_strip_customer_id(pay_provider_id).await;
        match user {
            Ok(Some(user)) => {
                let user = UserDto::try_from(&user)?;
                Ok(user)
            }
            Ok(None) => {
                tracing::info!("User with subscription provider id {} not found", pay_provider_id);
                Err(Error::NotFound("User not found".to_string()))
            },
            Err(e) => Err(e),
        }
    }

    pub async fn update(&self, updates: UpdateUserDto, user: &mut User) -> Result<UserDto> {
        user.update_profile(updates.first_name, updates.last_name, updates.phone, updates.photo_url);
        user.update(updates.status, user.role());

        let user = self.user_repo.update(user).await?;

        let user = UserDto::try_from(&user)?;
        Ok(user)
    }

    pub async fn delete(&self, id: &Uuid) -> Result<()> {
        self.user_repo.delete(id).await
    }
}


#[derive(Debug, Clone)]
pub struct AuthenticationService<A: Authenticator> {
    auth_client: Arc<A>
}

impl<A: Authenticator> AuthenticationService<A> {
    pub fn new(auth_client: Arc<A>) -> Self {
        Self { auth_client }
    }

    pub async fn authenticate(&self, token: &str) -> Result<AuthProviderData> {
        self.auth_client.authenticate(token).await
    }
}