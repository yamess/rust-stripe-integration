use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::user::value_objects::role::Role;
use crate::domain::user::value_objects::user_status::UserStatus;
use crate::prelude::*;


#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    email: String,
    firebase_id: String,
    strip_customer_id: String,
    status: UserStatus,
    role: Role,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    profile: Profile,

}
impl User {
    pub fn new(
        id: Uuid,
        email: String,
        firebase_id: String,
        strip_customer_id: String,
        status: UserStatus,
        role: Role,
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
        profile: Profile,
    ) -> Self {
        Self {
            id,
            email,
            firebase_id,
            strip_customer_id,
            status,
            role,
            created_at,
            updated_at,
            profile,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn firebase_id(&self) -> &str {
        &self.firebase_id
    }

    pub fn strip_customer_id(&self) -> &str {
        &self.strip_customer_id
    }

    pub fn status(&self) -> UserStatus {
        self.status
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        self.updated_at
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    pub fn update(&mut self, status: UserStatus, role: Role, updated_at: DateTime<Utc>) {
        self.status = status;
        self.role = role;
        self.updated_at = Some(updated_at);
    }

    pub fn update_profile(
        &mut self,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        photo_url: Option<String>,
        updated_at: DateTime<Utc>,
    ) {
        self.profile.update(first_name, last_name, phone, photo_url, updated_at);
    }
}


#[derive(Debug, Clone)]
pub struct Profile {
    id: i32,
    user_id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
    photo_url: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>
}

impl Profile {
    pub fn new(
        id: i32,
        user_id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        photo_url: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            user_id,
            first_name,
            last_name,
            phone,
            photo_url,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn first_name(&self) -> Option<&str> {
        self.first_name.as_deref()
    }

    pub fn last_name(&self) -> Option<&str> {
        self.last_name.as_deref()
    }

    pub fn phone(&self) -> Option<&str> {
        self.phone.as_deref()
    }

    pub fn photo_url(&self) -> Option<&str> {
        self.photo_url.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<DateTime<Utc> > {
        self.updated_at
    }

    pub fn update(
        &mut self,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        photo_url: Option<String>,
        updated_at: DateTime<Utc>,
    ) {
        self.first_name = first_name;
        self.last_name = last_name;
        self.phone = phone;
        self.photo_url = photo_url;
        self.updated_at = Some(updated_at);
    }
}