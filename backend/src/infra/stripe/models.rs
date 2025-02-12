use serde::{Deserialize, Serialize};
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::product::Product;
use crate::domain::payment::entities::product_price::ProductPrice;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCustomerResponse {
    pub data: Vec<Customer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProductResponse {
    pub data: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPriceResponse {
    pub data: Vec<ProductPrice>,
}