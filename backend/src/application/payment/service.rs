use std::sync::Arc;
use crate::application::payment::dto::{NewCheckoutSessionDto, NewCustomerDto, NewPortalDto, NewPriceDto, NewProductDto};
use crate::domain::payment::client::PaymentClient;
use crate::domain::payment::entities::checkout::{CheckoutSession, CheckoutSessionResponse};
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::portal::CustomerPortalSession;
use crate::domain::payment::entities::product::Product;
use crate::domain::payment::entities::product_price::ProductPrice;
use crate::domain::plans::value_objects::currency::Currency;
use crate::prelude::*;


#[derive(Clone)]
pub struct PaymentService<C> {
    client: Arc<C>,
}
impl<C: PaymentClient> PaymentService<C> {
    pub fn new(client: Arc<C>) -> Self {
        Self { client }
    }

    pub async fn create_customer(&self, new_customer: NewCustomerDto) -> Result<Customer> {
        let customer = Customer::try_from(new_customer)?;
        let result = self.client.create_customer(&customer).await?;
        Ok(result)
    }

    pub async fn get_customer(&self, email: &str) -> Result<Customer> {
        let result = self.client.get_customer(email).await?;
        Ok(result)
    }

    pub async fn create_product(&self, new_product: NewProductDto) -> Result<Product> {
        let product = Product::try_from(new_product)?;
        let result = self.client.create_product(&product).await?;
        Ok(result)
    }

    pub async fn get_product(&self, name: &str) -> Result<Product> {
        let result = self.client.get_product(name).await?;
        Ok(result)
    }

    pub async fn create_price(&self, new_price: NewPriceDto) -> Result<ProductPrice> {
        let price = ProductPrice::try_from(new_price)?;
        let result = self.client.create_price(&price).await?;
        Ok(result)
    }

    pub async fn search_prices(
        &self, currency: &Currency, product: &str, active: bool
    ) -> Result<Vec<ProductPrice>> {
        let result = self.client.search_prices(currency, product, active).await?;
        Ok(result)
    }

    pub async fn create_checkout_session(&self, new_checkout: NewCheckoutSessionDto) ->
                                                                                     Result<CheckoutSessionResponse> {
        let checkout = CheckoutSession::try_from(new_checkout)?;
        let result = self.client.create_checkout_session(&checkout).await?;
        Ok(result)
    }

    pub async fn create_portal_session(&self, new_portal: NewPortalDto) -> Result<CustomerPortalSession> {
        let portal = CustomerPortalSession::try_from(new_portal)?;
        let result = self.client.create_portal_session(&portal).await?;
        Ok(result)
    }
}
