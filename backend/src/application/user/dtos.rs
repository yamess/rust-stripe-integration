use serde::Deserialize;
use crate::domain::user::value_objects::role::Role;


#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub firebase_id: String,
    pub stripe_customer_id: String,
    pub role: Role,
}