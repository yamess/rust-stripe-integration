use crate::domain::user::entities::Profile;
use crate::prelude::*;
use crate::schema;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::profiles)]
pub struct CreateProfileModel {
    pub user_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub photo_url: Option<String>,
}
impl TryFrom<&Profile> for CreateProfileModel {
    type Error = Error;

    fn try_from(profile: &Profile) -> Result<Self> {
        Ok(Self {
            user_id: profile.user_id(),
            first_name: profile.first_name().map(|s| s.to_string()),
            last_name: profile.last_name().map(|s| s.to_string()),
            phone: profile.phone().map(|s| s.to_string()),
            photo_url: profile.photo_url().map(|s| s.to_string()),
        })
    }
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(belongs_to(UserModel, foreign_key = user_id))]
#[diesel(table_name = schema::profiles, check_for_backend(diesel::pg::Pg))]
pub struct ProfileModel {
    pub id: i32,
    pub user_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<&Profile> for ProfileModel {
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

impl TryFrom<ProfileModel> for Profile {
    type Error = Error;

    fn try_from(model: ProfileModel) -> Result<Self> {
        Ok(Profile::construct(
            model.id,
            model.user_id,
            model.first_name,
            model.last_name,
            model.phone,
            model.photo_url,
            model.created_at,
            model.updated_at,
        ))
    }
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = schema::profiles)]
#[diesel(belongs_to(UserModel, foreign_key = user_id))]
pub struct UpdateProfileModel {
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
    photo_url: Option<String>,
    updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<&Profile> for UpdateProfileModel {
    type Error = Error;

    fn try_from(profile: &Profile) -> Result<Self> {
        Ok(Self {
            first_name: profile.first_name().map(|s| s.to_string()),
            last_name: profile.last_name().map(|s| s.to_string()),
            phone: profile.phone().map(|s| s.to_string()),
            photo_url: profile.photo_url().map(|s| s.to_string()),
            updated_at: profile.updated_at(),
        })
    }
}
