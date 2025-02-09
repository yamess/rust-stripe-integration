use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentSession {
    url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPrice {
    amount: i32,
    currency: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentCustomer {
    email: String
}