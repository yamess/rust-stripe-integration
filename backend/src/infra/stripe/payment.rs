use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::domain::payment::entities::checkout::{CheckoutSession, };
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::portal::{CustomerPortalSession, CustomerPortalSessionResponse};
use crate::domain::payment::client::PaymentClient;
use crate::domain::user::entities::User;
use crate::infra::stripe::models::{CheckoutSessionForm, GetCustomerResponse};
use crate::prelude::*;


#[derive(Clone)]
pub struct StripePaymentClient {
    http: Arc<reqwest::Client>,
    secret_key: String,
    base_url: String,
    headers: reqwest::header::HeaderMap,
}

impl StripePaymentClient {
    pub fn new(
        secret_key: &str,
        http_client: Arc<reqwest::Client>,
        base_url: &str,
    ) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
        Self {
            http: http_client,
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
            headers,
        }
    }
}

impl PaymentClient for StripePaymentClient {
    async fn create_customer(&self, customer: &Customer) -> Result<Customer> {
       let url = format!("{}/customers", self.base_url);
       let response = self.http.post(&url)
           .basic_auth(&self.secret_key, Some(""))
           .headers(self.headers.clone())
//           .header("Idempotency-Key", customer.email())
           .form(&customer)
           .send()
           .await?;

       let status = response.status();
       if status.is_success() {
           let customer = response.json::<Customer>().await.map_err(|e| {
               tracing::error!("Failed to create customer: {:?}", e);
               Error::DeserializationError("Failed to create customer".to_string())
           })?;

           tracing::info!("Created customer: {:?}", customer);
           Ok(customer)
       } else {
           let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
           tracing::error!("Failed to create customer (HTTP {}): {}", status, error_body);
           let code = status.as_u16();
           Err(Error::ApiError(code, error_body))
       }
   }
    async fn get_customer(&self, email: &str) -> Result<Customer> {
       let url = format!("{}/customers/search", self.base_url);
       let query = format!("email:'{}'", email);
       let response = self.http.get(&url)
           .basic_auth(&self.secret_key, Some(""))
           .query(&[("query", query)])
           .send()
           .await?;

       let status = response.status();

       if status.is_success() {
           let customer = response.json::<GetCustomerResponse>().await.map_err(|e| {
               tracing::error!("Failed to serialize customer data: {:?}", e);
               Error::DeserializationError("Failed to get customer".to_string())
           })?;
           let customer = customer.data.into_iter().next().ok_or_else(|| {
               tracing::error!("Failed to get customer: Customer not found");
               Error::NotFound("Customer not found".to_string())
           })?;
           Ok(customer)
       } else {
           let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
           tracing::error!("Failed to get customer (HTTP {}): {}", status, error_body);
           let code = status.as_u16();
           Err(Error::ApiError(code, error_body))
       }
   }
    async fn create_checkout_session(&self, checkout: &CheckoutSession) -> Result<String> {
        let url = format!("{}/checkout/sessions", self.base_url);
        let form_data = CheckoutSessionForm::try_from(checkout)?;

        let response = self.http
            .post(&url)
            .headers(self.headers.clone())
            .basic_auth(&self.secret_key, Some(""))
            .form(&form_data.data)
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            let response = response.json::<Value>().await?;
            let session = response["url"].as_str().ok_or_else(|| {
                tracing::error!("Failed to create checkout session: URL not found");
                Error::DeserializationError("Failed to create checkout session".to_string())
            })?.to_string();
            Ok(session)
        } else {
            let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            tracing::error!("Failed to create checkout session (HTTP {}): {}", status, error_body);
            let code = status.as_u16();
            Err(Error::ApiError(code, error_body))
        }
    }
    async fn create_portal_session(&self, portal: &CustomerPortalSession) -> Result<String>{
        let url = format!("{}/billing_portal/sessions", self.base_url);
        let response = self.http
            .post(&url)
            .basic_auth(&self.secret_key, Some(""))
            .form(&portal)
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            let response = response.json::<Value>().await.map_err(|e| {
                tracing::error!("Failed to create billing portal session: {:?}", e);
                Error::DeserializationError("Failed to create billing portal session".to_string())
            })?;
            let session = response["url"].as_str().ok_or_else(|| {
                tracing::error!("Failed to create billing portal session: URL not found");
                Error::DeserializationError("Failed to create billing portal session".to_string())
            })?.to_string();
            Ok(session)
        } else {
            let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            tracing::error!("Failed to create billing portal session (HTTP {}): {}", status, error_body);
            let code = status.as_u16();
            Err(Error::ApiError(code, error_body))
        }
    }
}