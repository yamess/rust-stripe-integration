use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::domain::payment::entities::checkout::CheckoutSession;
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::portal::CustomerPortalSession;
use crate::domain::payment::entities::product::Product;
use crate::domain::payment::entities::product_price::ProductPrice;
use crate::domain::payment::client::PaymentClient;
use crate::domain::plans::value_objects::currency::Currency;
use crate::domain::user::entities::User;
use crate::infra::stripe::models::{GetCustomerResponse, GetProductResponse, SearchPriceResponse};
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

    async fn create_product(&self, product: &Product) -> Result<Product> {
        let url = format!("{}/products", self.base_url);
        let response = self.http.post(&url)
            .basic_auth(&self.secret_key, Some(""))
            .headers(self.headers.clone())
            .form(&product)
            .send()
            .await?;

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

    async fn get_product(&self, name: &str) -> Result<Product> {
        let url = format!("{}/products/search",self.base_url);
        let query = format!("name:'{}'", name);
        let response = self.http.get(&url)
            .basic_auth(&self.secret_key, Some(""))
            .query(&[("query", query)])
            .send()
            .await?;
        let status = response.status();

        if status.is_success() {
            let product = response.json::<GetProductResponse>().await.map_err(|e| {
                tracing::error!("Failed to serialize product data: {:?}", e);
                Error::DeserializationError("Failed to get product".to_string())
            })?;
            let product = product.data.into_iter().next().ok_or_else(|| {
                tracing::error!("Failed to get product: Product not found");
                Error::NotFound("Product not found".to_string())
            })?;
            Ok(product)
        } else {
            let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            tracing::error!("Failed to get product (HTTP {}): {}", status, error_body);
            let code = status.as_u16();
            Err(Error::ApiError(code, error_body))
        }

    }
    
    async fn create_price(&self, price: &ProductPrice) -> Result<ProductPrice> {
        let url = format!("{}/prices", self.base_url);

        let response = self.http.post(&url)
            .basic_auth(&self.secret_key, Some(""))
            .headers(self.headers.clone())
            .form(&price)
            .send()
            .await?;

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

    async fn search_prices(&self, currency: &Currency, active: bool) -> Result<Vec<ProductPrice>>{
        let url = format!("{}/prices/search", self.base_url);
        let query = format!("currency:'{}' active:'{}'", currency, active);
        let response = self.http.get(&url)
            .basic_auth(&self.secret_key, Some(""))
            .query(&[("query", query)])
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            let prices = response.json::<SearchPriceResponse>().await.map_err(|e| {
                tracing::error!("Failed to deserialize price data: {:?}", e);
                Error::DeserializationError("Failed to get prices".to_string())
            })?;
            Ok(prices.data)
        } else {
            let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            tracing::error!("Failed to get prices (HTTP {}): {}", status, error_body);
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
            .await?;

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

    async fn create_portal_session(&self, portal: &CustomerPortalSession) -> Result<CustomerPortalSession>{
        let url = format!("{}/billing_portal/sessions", self.base_url);
        let response = self.http
            .post(&url)
            .basic_auth(&self.secret_key, Some(""))
            .json(&portal)
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            let session = response.json::<CustomerPortalSession>().await.map_err(|e| {
                tracing::error!("Failed to create billing portal session: {:?}", e);
                Error::DeserializationError("Failed to create billing portal session".to_string())
            })?;
            Ok(session)
        } else {
            let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            tracing::error!("Failed to create billing portal session (HTTP {}): {}", status, error_body);
            let code = status.as_u16();
            Err(Error::ApiError(code, error_body))
        }
    }
}