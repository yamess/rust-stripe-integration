use std::sync::Arc;
use serde_json::Value;
use uuid::Uuid;
use crate::domain::billing::entities::plan::Plan;
use crate::domain::services::PaymentProviderService;
use crate::domain::user::entities::User;
use crate::prelude::*;


#[derive(Clone)]
pub struct StripeClient {
    http: Arc<reqwest::Client>,
    secret_key: String,
    base_url: String,
}
impl StripeClient {
    pub fn new(secret_key: String, http_client: Arc<reqwest::Client>) -> Self {
        Self {
            http: http_client,
            secret_key,
            base_url: "https://api.stripe.com/v1".to_string(),
        }
    }
}

impl PaymentProviderService for StripeClient {
    async fn create_customer(&self, user: &User) -> Result<Value> {
        let url = format!("{}/customers", self.base_url);
        let response = self.http.post(&url)
            .basic_auth(&self.secret_key, Some(""))
            .json(&serde_json::json!({
                "email": user.email(),
                "name": user.profile().full_name(),
            }))
            .send()
            .await.map_err(|e| {
                tracing::error!("Failed to create customer: {:?}", e);
                Error::TransportError("Failed to create customer".to_string())
            })?;

        let status = response.status();
        if status.is_success() {
            let customer = response.json::<serde_json::Value>().await.map_err(|e| {
                tracing::error!("Failed to create customer: {:?}", e);
                Error::DeserializationError("Failed to create customer".to_string())
            })?;
            Ok(customer)
        } else {
            let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            tracing::error!("Failed to create customer (HTTP {}): {}", status, error_body);
            let code = status.as_u16();
            Err(Error::ApiError(code, error_body))
        }
    }

    async fn create_price(&self, product_id: &str, plan: &Plan) -> Result<Value> {
        let url = format!("{}/prices", self.base_url);
        let trial = if plan.trial_period_days() > 0 {
            Some(plan.trial_period_days())
        } else {
            None
        };
        let response = self.http.post(&url)
            .basic_auth(&self.secret_key, Some(""))
            .json(&serde_json::json!({
                "name": plan.name(),
                "currency": plan.currency(),
                "unit_amount": plan.price().value(),
                "recurring": {
                    "interval": plan.billing_cycle().interval().0,
                    "interval_count": plan.billing_cycle().interval().1,
                    "trial_period_days": trial,
                },
                "product": product_id,
            }))
            .send()
            .await.map_err(|e| {
                tracing::error!("Failed to create price: {:?}", e);
                Error::TransportError("Failed to create price".to_string())
            })?;

        let status = response.status();

        if status.is_success() {
            let price = response.json::<serde_json::Value>().await.map_err(|e| {
                tracing::error!("Failed to create price: {:?}", e);
                Error::DeserializationError("Failed to create price".to_string())
            })?;
            Ok(price)
        } else {
            let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            tracing::error!("Failed to create price (HTTP {}): {}", status, error_body);
            let code = status.as_u16();
            Err(Error::ApiError(code, error_body))
        }
    }
}