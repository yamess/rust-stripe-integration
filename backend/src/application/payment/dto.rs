use serde::{Deserialize, Serialize};
use crate::domain::payment::entities::checkout::{CheckoutSession, LineItem};
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::portal::CustomerPortalSession;
use crate::domain::payment::entities::product::Product;
use crate::domain::payment::entities::product_price::{ProductPrice, Recurring};
use crate::domain::payment::value_objects::ui_mode::UiMode;
use crate::domain::plans::value_objects::currency::Currency;
use crate::domain::plans::value_objects::price::Price;
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

//*******************************************//
//************** NewProductDto **************//
//*******************************************//
#[derive(Debug, Deserialize)]
pub struct NewProductDto {
    pub name: String,
}
impl NewProductDto {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
impl TryFrom<NewProductDto> for Product {
    type Error = Error;
    fn try_from(dto: NewProductDto) -> Result<Self> {
        Ok(Product::new(dto.name))
    }
}

//*******************************************//
//*************** NewPriceDto ***************//
//*******************************************//
#[derive(Debug, Deserialize)]
pub struct NewPriceDto {
    active: bool,
    currency: Currency,
    unit_amount: Price,
    product: String,
    recurring: Recurring
}
impl NewPriceDto {
    pub fn new(active: bool, currency: Currency, unit_amount: Price, product: String, recurring: Recurring) -> Self {
        Self { active, currency, unit_amount, product, recurring }
    }
}
impl TryFrom<NewPriceDto> for ProductPrice {
    type Error = Error;
    fn try_from(dto: NewPriceDto) -> Result<Self> {
        Ok(ProductPrice::new(dto.active, dto.currency, dto.unit_amount, dto.product, dto.recurring))
    }
}

#[derive(Debug, Deserialize)]
pub struct PriceSearchQuery {
    pub currency: Currency,
    pub active: bool
}

//***************************************************//
//************** NewCheckoutSessionDto **************//
//***************************************************//
#[derive(Debug, Deserialize)]
pub struct NewCheckoutSessionDto {
    customer: String,
    customer_email: String,
    line_items: Vec<LineItem>,
    mode: String,
    ui_mode: UiMode,
    return_url: Option<String>,
    success_url: Option<String>,
    cancel_url: Option<String>,
}
impl NewCheckoutSessionDto {
    pub fn new(
        customer: String,
        customer_email: String,
        line_items: Vec<LineItem>,
        mode: String,
        ui_mode: UiMode,
        return_url: Option<String>,
        success_url: Option<String>,
        cancel_url: Option<String>,
    ) -> Self {
        Self {
            customer,
            customer_email,
            line_items,
            mode,
            ui_mode,
            return_url,
            success_url,
            cancel_url,
        }
    }
}
impl TryFrom<NewCheckoutSessionDto> for CheckoutSession {
    type Error = Error;
    fn try_from(dto: NewCheckoutSessionDto) -> Result<Self> {
        Ok(CheckoutSession::new(
            dto.customer,
            dto.customer_email,
            dto.line_items,
            dto.mode,
            dto.ui_mode,
            dto.return_url,
            dto.success_url,
            dto.cancel_url,
        ))
    }
}

//*******************************************//
//*************** NewPortalDto ***************//
//*******************************************//
#[derive(Debug, Deserialize)]
pub struct NewPortalDto {
    customer: String,
    return_url: String,
}
impl NewPortalDto {
    pub fn new(customer: String, return_url: String) -> Self {
        Self { customer, return_url }
    }
}
impl TryFrom<NewPortalDto> for CustomerPortalSession {
    type Error = Error;
    fn try_from(dto: NewPortalDto) -> Result<Self> {
        Ok(CustomerPortalSession::new(dto.customer, dto.return_url))
    }
}