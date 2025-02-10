use std::sync::Arc;
use serde_json::Value;
use uuid::Uuid;
use crate::domain::payment::old_entities::PaymentSession;
use crate::domain::payment::service::PaymentService;
use crate::domain::plans::entities::Plan;
use crate::domain::user::entities::User;
use crate::prelude::*;


#[derive(Clone)]
pub struct StripePaymentService {
    http: Arc<reqwest::Client>,
    secret_key: String,
    base_url: String,
    success_page: String,
    cancel_page: String,
}

impl StripePaymentService {
    pub fn new(
        secret_key: &str,
        success_page: &str,
        cancel_page: &str,
        http_client: Arc<reqwest::Client>
    ) -> Self {
        Self {
            http: http_client,
            secret_key: secret_key.to_string(),
            base_url: "https://api.stripe.com/v1".to_string(),
            success_page: success_page.to_string(),
            cancel_page: cancel_page.to_string(),
        }
    }
}

impl PaymentService for StripePaymentService {
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

    async fn create_checkout_session(&self, user: &User, plan: &Plan) -> Result<PaymentSession> {
        let url = format!("{}/checkout/sessions", self.base_url);
        let response = self.http
            .post(&url)
            .basic_auth(&self.secret_key, Some(""))
            .json(&serde_json::json!({
                "customer_email": user.email(),
                "payment_method_types": ["card"],
                "line_items": [{
                    "price": plan.stripe_price_id(),
                    "quantity": 1,
                }],
                "mode": "subscription",
                "success_url": self.success_page,
                "cancel_url": self.cancel_page,
            }))
            .send()
            .await.map_err(|e| {
                tracing::error!("Failed to create checkout session: {:?}", e);
                Error::TransportError("Failed to create checkout session".to_string())
            })?;

        let status = response.status();

        if status.is_success() {
            let session = response.json::<PaymentSession>().await.map_err(|e| {
                tracing::error!("Failed to create checkout session: {:?}", e);
                Error::DeserializationError("Failed to create checkout session".to_string())
            })?;
            Ok(session)
        } else {
            let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            tracing::error!("Failed to create checkout session (HTTP {}): {}", status, error_body);
            let code = status.as_u16();
            Err(Error::ApiError(code, error_body))
        }
    }
}