use serde::{Deserialize, Serialize};
use crate::domain::payment::value_objects::ui_mode::UiMode;

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItem {
    price: String,
    quantity: i32,
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
        return_url: Option<String>,
        success_url: Option<String>,
        cancel_url: Option<String>,
    ) -> Self {
        Self {
            id,
            customer,
            customer_email,
            line_items,
            mode,
            ui_mode: UiMode::Embedded,
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