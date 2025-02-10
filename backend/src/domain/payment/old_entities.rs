use serde::{Deserialize, Serialize};
use stripe::Recurring;
use crate::domain::plans::value_objects::currency::Currency;







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