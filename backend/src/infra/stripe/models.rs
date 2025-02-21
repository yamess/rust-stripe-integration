use crate::domain::payment::entities::checkout::CheckoutSession;
use crate::domain::payment::entities::customer::Customer;
use crate::infra::constants::{CHECKOUT_MODE, TRIAL_PERIOD_DAYS, UI_MODE};
use crate::prelude::*;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};
use serde_urlencoded::Deserializer;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCustomerResponse {
    pub data: Vec<Customer>,
}

#[derive(Debug)]
pub struct CheckoutSessionForm {
    pub data: Vec<(String, String)>,
}
impl TryFrom<&CheckoutSession> for CheckoutSessionForm {
    type Error = Error;

    fn try_from(checkout: &CheckoutSession) -> Result<Self> {
        let mut data = vec![];
        data.push(("customer".to_string(), checkout.customer().to_string()));
        data.push(("mode".to_string(), CHECKOUT_MODE.to_string()));
        data.push(("ui_mode".to_string(), UI_MODE.to_string()));

        if let Some(success_url) = checkout.success_url() {
            data.push(("success_url".to_string(), success_url.to_string()));
        }
        if let Some(cancel_url) = checkout.cancel_url() {
            data.push(("cancel_url".to_string(), cancel_url.to_string()));
        }
        for (index, item) in checkout.line_items().iter().enumerate() {
            let price_key = format!("line_items[{}][price]", index);
            let quantity_key = format!("line_items[{}][quantity]", index);
            data.push((price_key, item.price.to_string()));
            data.push((quantity_key, item.quantity.to_string()));
        }
        if TRIAL_PERIOD_DAYS > 0 {
            data.push((
                "subscription_data[trial_period_days]".to_string(),
                TRIAL_PERIOD_DAYS.to_string(),
            ));
        }
        Ok(CheckoutSessionForm { data })
    }
}

//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct LineItemForm {
//     pub price: String,
//     pub quantity: i32,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct SubscriptionDataForm {
//     pub trial_period_days: i32,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct CheckoutSessionForm {
//     pub customer: Option<String>,
//     pub customer_email: Option<String>,
//     pub line_items: Vec<LineItemForm>,
//     pub mode: String,
//     pub ui_mode: String,
//     pub return_url: Option<String>,
//     pub success_url: Option<String>,
//     pub cancel_url: Option<String>,
//     pub subscription_data: Option<SubscriptionDataForm>,
// }
// impl TryFrom<&CheckoutSession> for CheckoutSessionForm {
//     type Error = Error;
//     fn try_from(checkout: &CheckoutSession) -> Result<Self> {
//         let subscription_data = if TRIAL_PERIOD_DAYS > 0 {
//             Some(SubscriptionDataForm {
//                 trial_period_days: TRIAL_PERIOD_DAYS,
//             })
//         } else {
//             None
//         };
//         Ok(CheckoutSessionForm {
//             customer: Some(checkout.customer().to_string()),
//             customer_email: Some(checkout.customer_email().to_string()),
//             line_items: checkout.line_items().iter().map(|item| LineItemForm {
//                 price: item.price.to_string(),
//                 quantity: item.quantity,
//             }).collect(),
//             mode: checkout.mode().to_string(),
//             ui_mode: checkout.ui_mode().to_string(),
//             return_url: checkout.return_url().map(|url| url.to_string()),
//             success_url: checkout.success_url().map(|url| url.to_string()),
//             cancel_url: checkout.cancel_url().map(|url| url.to_string()),
//             subscription_data,
//         })
//     }
// }
