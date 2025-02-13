use serde::{Deserialize, Serialize};
use crate::domain::payment::value_objects::ui_mode::UiMode;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItem {
    pub price: String,
    pub quantity: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSession {
    #[serde(skip_serializing)]
    id: String,
    customer: String,
    customer_email: String,
    line_items: Vec<LineItem>,
    mode: String,
    #[serde(skip_deserializing)]
    ui_mode: UiMode,
    return_url: Option<String>,
    success_url: Option<String>,
    cancel_url: Option<String>,
    #[serde(skip_serializing)]
    url: String,
}
impl CheckoutSession {
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
            id: Default::default(),
            customer,
            customer_email,
            line_items,
            mode,
            ui_mode,
            return_url,
            success_url,
            cancel_url,
            url: "".to_string(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn customer(&self) -> &str {
        &self.customer
    }

    pub fn customer_email(&self) -> &str {
        &self.customer_email
    }

    pub fn line_items(&self) -> &Vec<LineItem> {
        &self.line_items
    }

    pub fn mode(&self) -> &str {
        &self.mode
    }

    pub fn ui_mode(&self) -> &UiMode {
        &self.ui_mode
    }

    pub fn return_url(&self) -> Option<&str> {
        self.return_url.as_deref()
    }

    pub fn success_url(&self) -> Option<&str> {
        self.success_url.as_deref()
    }

    pub fn cancel_url(&self) -> Option<&str> {
        self.cancel_url.as_deref()
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSessionResponse {
    pub id: String,
    pub url: String,
    pub amount_subtotal: i64,
    pub amount_total: i64,
    pub currency: String,
}
impl CheckoutSessionResponse {
    pub fn new(id: String, url: String, amount_subtotal: i64, amount_total: i64, currency: String) -> Self {
        Self {
            id,
            url,
            amount_subtotal,
            amount_total,
            currency,
        }
    }
}