use diesel;
use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;
use crate::schema::profiles;
use chrono::{DateTime, Utc};
use crate::domain::user::entities::Profile;
use crate::prelude::*;


#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = profiles)]
pub struct CreateProfileModel {
    user_id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
    photo_url: Option<String>,
}



#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(belongs_to(UserModel, foreign_key = user_id))]
#[diesel(table_name = profiles)]
pub struct ProfileModel {
    id: i32,
    user_id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
    photo_url: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
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
        Ok(Profile::new(
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
