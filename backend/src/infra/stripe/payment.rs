use std::sync::Arc;
use serde_json::Value;
use uuid::Uuid;
use crate::domain::payment::entities::checkout::CheckoutSession;
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::product::Product;
use crate::domain::payment::entities::product_price::ProductPrice;
use crate::domain::payment::old_entities::PaymentSession;
use crate::domain::payment::service::PaymentService;
use crate::domain::user::entities::User;
use crate::prelude::*;


#[derive(Clone)]
pub struct StripePaymentService {
    http: Arc<reqwest::Client>,
    secret_key: String,
    base_url: String,
}

impl StripePaymentService {
    pub fn new(
        secret_key: &str,
        http_client: Arc<reqwest::Client>,
        base_url: &str,
    ) -> Self {
        Self {
            http: http_client,
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
        }
    }
}

impl PaymentService for StripePaymentService {
    async fn create_customer(&self, customer: &Customer) -> Result<Customer> {
        let url = format!("{}/customers", self.base_url);
        let response = self.http.post(&url)
            .basic_auth(&self.secret_key, Some(""))
            .json(&customer)
            .send()
            .await.map_err(|e| {
                tracing::error!("Failed to create customer: {:?}", e);
                Error::TransportError("Failed to create customer".to_string())
            })?;

        let status = response.status();
        if status.is_success() {
            let customer = response.json::<Customer>().await.map_err(|e| {
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

    async fn create_product(&self, product: &Product) -> Result<Product> {
        let url = format!("{}/products", self.base_url);
        let response = self.http.post(&url)
            .basic_auth(&self.secret_key, Some(""))
            .json(&product)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to create product: {:?}", e);
                Error::TransportError("Failed to create product".to_string())
            })?;
        let status = response.status();
        if status.is_success() {
            let product = response.json::<Product>().await.map_err(|e| {
                tracing::error!("Failed to create product: {:?}", e);
                Error::DeserializationError("Failed to create product".to_string())
            })?;
            Ok(product)
        } else {
            let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            tracing::error!("Failed to create product (HTTP {}): {}", status, error_body);
            let code = status.as_u16();
            Err(Error::ApiError(code, error_body))
        }
    }

    async fn create_price(&self, price: &ProductPrice) -> Result<ProductPrice> {
        let url = format!("{}/prices", self.base_url);

        let response = self.http.post(&url)
            .basic_auth(&self.secret_key, Some(""))
            .json(&price)
            .send()
            .await.map_err(|e| {
                tracing::error!("Failed to create price: {:?}", e);
                Error::TransportError("Failed to create price".to_string())
            })?;

        let status = response.status();

        if status.is_success() {
            let price = response.json::<ProductPrice>().await.map_err(|e| {
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

    async fn create_checkout_session(&self, checkout: &CheckoutSession) -> Result<CheckoutSession> {
        let url = format!("{}/checkout/sessions", self.base_url);
        let response = self.http
            .post(&url)
            .basic_auth(&self.secret_key, Some(""))
            .json(&checkout)
            .send()
            .await.map_err(|e| {
                tracing::error!("Failed to create checkout session: {:?}", e);
                Error::TransportError("Failed to create checkout session".to_string())
            })?;

        let status = response.status();

        if status.is_success() {
            let session = response.json::<CheckoutSession>().await.map_err(|e| {
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