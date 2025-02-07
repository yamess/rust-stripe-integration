use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::user::entities::{Profile, User};
use crate::domain::user::value_objects::role::Role;
use crate::domain::user::value_objects::user_status::UserStatus;
use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct NewUserDto {
    pub email: String,
}
impl TryFrom<NewUserDto> for User {
    type Error = Error;

    fn try_from(new_user: NewUserDto) -> Result<Self> {
        Ok(User::new(
            new_user.email,
            "".to_string(),
            "".to_string(),
        ))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UserDto {
    pub id: Uuid,
    pub email: String,
    pub firebase_id: String,
    pub stripe_customer_id: String,
    pub status: UserStatus,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub profile: ProfileDto
}
impl TryFrom<&User> for UserDto {
    type Error = Error;

    fn try_from(user: &User) -> Result<Self> {
        Ok(Self {
            id: user.id(),
            email: user.email().to_string(),
            firebase_id: user.firebase_id().to_string(),
            stripe_customer_id: user.stripe_customer_id().to_string(),
            status: user.status(),
            role: user.role(),
            created_at: user.created_at(),
            updated_at: user.updated_at().unwrap_or_else(|| Utc::now()),
            profile: ProfileDto::try_from(user.profile())?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub status: UserStatus,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub photo_url: Option<String>,
}


#[derive(Debug, Clone, Serialize)]
pub struct ProfileDto {
    pub id: i32,
    pub user_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl TryFrom<&Profile> for ProfileDto {
    type Error = Error;

    fn try_from(profile: &Profile) -> Result<Self> {
        Ok(Self {
            id: profile.id(),
            user_id: profile.user_id(),
            first_name: profile.first_name().map(|s| s.to_string()),
            last_name: profile.last_name().map(|s| s.to_string()),
            phone: profile.phone().map(|s| s.to_string()),
            photo_url: profile.photo_url().map(|s| s.to_string()),
            created_at: profile.created_at(),
            updated_at: profile.updated_at(),
        })
    }
}