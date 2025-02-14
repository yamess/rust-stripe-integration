use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItem {
    pub price: String,
    pub quantity: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSession {
    customer: String,
    line_items: Vec<LineItem>,
    success_url: Option<String>,
    cancel_url: Option<String>,
}
impl CheckoutSession {
    pub fn new(
        customer: String,
        line_items: Vec<LineItem>,
        success_url: Option<String>,
        cancel_url: Option<String>,
    ) -> Self {
        Self {
            customer,
            line_items,
            success_url,
            cancel_url,
        }
    }

    pub fn customer(&self) -> &str {
        &self.customer
    }

    pub fn line_items(&self) -> &Vec<LineItem> {
        &self.line_items
    }

    pub fn success_url(&self) -> Option<&str> {
        self.success_url.as_deref()
    }

    pub fn cancel_url(&self) -> Option<&str> {
        self.cancel_url.as_deref()
    }
    pub fn add_line_item(&mut self, item: LineItem) {
        self.line_items.push(item);
    }
    pub fn remove_line_item(&mut self, index: usize) {
        self.line_items.remove(index);
    }
    pub fn clear_line_items(&mut self) {
        self.line_items.clear();
    }
    pub fn update_line_item(&mut self, index: usize, item: LineItem) {
        self.line_items[index] = item;
    }
}
