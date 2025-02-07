use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;
use crate::domain::user::value_objects::role::Role;
use crate::domain::user::value_objects::user_status::UserStatus;


#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    email: String,
    firebase_id: String,
    stripe_customer_id: String,
    status: UserStatus,
    role: Role,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    profile: Profile,
}
impl User {
    pub fn new(
        email: String,
        firebase_id: String,
        stripe_customer_id: String,
    ) -> Self {
        Self {
            id: Uuid::nil(), // Real value will set by the repository database
            email,
            firebase_id,
            stripe_customer_id,
            status: UserStatus::Active,
            role: Role::User,
            created_at: Utc::now(),
            updated_at: None,
            profile: Profile::default(),
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

    pub fn stripe_customer_id(&self) -> &str {
        &self.stripe_customer_id
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

    pub fn update(&mut self, status: UserStatus, role: Role) {
        self.status = status;
        self.role = role;
        self.updated_at = Some(Utc::now());
    }

    pub fn update_profile(
        &mut self,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        photo_url: Option<String>,
    ) {
        self.profile.update(first_name, last_name, phone, photo_url);
    }

    pub fn construct(
        id: Uuid,
        email: String,
        firebase_id: String,
        stripe_customer_id: String,
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
            stripe_customer_id,
            status,
            role,
            created_at,
            updated_at,
            profile,
        }
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
    ) {
        self.first_name = first_name;
        self.last_name = last_name;
        self.phone = phone;
        self.photo_url = photo_url;
        self.updated_at = Some(Utc::now());
    }

    pub fn construct(
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
}


impl Default for Profile {
    fn default() -> Self {
        Self {
            id: 0,
            user_id: Uuid::nil(),
            first_name: None,
            last_name: None,
            phone: None,
            photo_url: None,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthProviderData {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub photo_url: Option<String>,
}
impl AuthProviderData {
    pub fn new(id: String, email: String, name: Option<String>, photo_url: Option<String>) -> Self {
        Self {
            id,
            email,
            name,
            photo_url,
        }
    }
}