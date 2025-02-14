use serde::{Deserialize, Serialize};
use crate::domain::payment::entities::checkout::{LineItem};
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::portal::CustomerPortalSession;
use crate::prelude::*;

//*******************************************//
//************** NewCustomerDto **************//
//*******************************************//
#[derive(Debug, Deserialize)]
pub struct NewCustomerDto {
    pub email: String,
    pub name: String
}
impl NewCustomerDto {
    pub fn new(email: String, name: String) -> Self {
        Self { email, name }
    }
}
impl TryFrom<NewCustomerDto> for Customer {
    type Error = Error;
    fn try_from(dto: NewCustomerDto) -> Result<Self> {
        Ok(Customer::new(dto.email, dto.name))
    }
}

//***************************************************//
//************** NewCheckoutSessionDto **************//
//***************************************************//
#[derive(Debug, Deserialize)]
pub struct NewCheckoutSessionDto {
    pub line_items: Vec<LineItem>,
    pub success_url: Option<String>,
    pub cancel_url: Option<String>,
}
impl NewCheckoutSessionDto {
    pub fn new(
        line_items: Vec<LineItem>,
        success_url: Option<String>,
        cancel_url: Option<String>,
    ) -> Self {
        Self {
            line_items,
            success_url,
            cancel_url,
        }
    }
}

//*******************************************//
//*************** NewPortalDto ***************//
//*******************************************//
#[derive(Debug, Deserialize)]
pub struct NewPortalDto {
    pub return_url: String,
}


//*****************************************//
//*************** SessionDto **************//
//*****************************************//
#[derive(Debug, Serialize)]
pub struct SessionDto {
    url: String,
}
impl SessionDto {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}