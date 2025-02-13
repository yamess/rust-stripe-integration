use std::cmp::PartialEq;
use std::str::FromStr;
use crate::application::payment::dto::{NewCheckoutSessionDto, NewCustomerDto, NewPortalDto, NewPriceDto, NewProductDto, PriceSearchQuery};
use crate::application::payment::service::PaymentService;
use crate::domain::payment::client::PaymentClient;
use crate::domain::payment::entities::checkout::CheckoutSession;
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::portal::CustomerPortalSession;
use crate::domain::payment::entities::product::Product;
use crate::domain::payment::entities::product_price::ProductPrice;
use crate::domain::plans::value_objects::currency::Currency;
use crate::prelude::*;

//************************************************//
//            Create Customer Use Cases                   //
//************************************************//
#[derive(Clone)]
pub struct CreateCustomerUseCase<C> {
    service: PaymentService<C>,
}
impl <C: PaymentClient> CreateCustomerUseCase<C> {
    pub fn new(service: PaymentService<C>) -> Self {
        Self { service }
    }

    pub async fn execute(&self, new_customer: NewCustomerDto) -> Result<Customer> {
        let result = self.service.get_customer(&new_customer.email).await;
        match result {
            Ok(customer) => Ok(customer),
            Err(Error::NotFound(_)) => self.service.create_customer(new_customer).await,
            Err(e) => Err(e),
        }

    }
}

pub struct GetCustomerUseCase<C> {
    service: PaymentService<C>,
}
impl <C: PaymentClient> GetCustomerUseCase<C> {
    pub fn new(service: PaymentService<C>) -> Self {
        Self { service }
    }

    pub async fn execute(&self, email: &str) -> Result<Customer> {
        self.service.get_customer(email).await
    }
}


//*******************************************************//
//            Create Product Use Cases                   //
//*******************************************************//
#[derive(Clone)]
pub struct CreateProductUseCase<C> {
    service: PaymentService<C>,
}
impl <C: PaymentClient> CreateProductUseCase<C> {
    pub fn new(service: PaymentService<C>) -> Self {
        Self { service }
    }

    pub async fn execute(&self, new_product: NewProductDto) -> Result<Product> {
        let result = self.service.get_product(&new_product.name).await;
        match result {
            Ok(product) => Ok(product),
            Err(Error::NotFound(_)) => self.service.create_product(new_product).await,
            Err(e) => Err(e),
        }
    }
}

#[derive(Clone)]
pub struct GetProductUseCase<C> {
    service: PaymentService<C>,
}
impl <C: PaymentClient> GetProductUseCase<C> {
    pub fn new(service: PaymentService<C>) -> Self {
        Self { service }
    }

    pub async fn execute(&self, name: &str) -> Result<Product> {
        self.service.get_product(name).await
    }
}


//*******************************************************//
//              Create Price Use Cases                   //
//*******************************************************//
#[derive(Clone)]
pub struct CreatePriceUseCase<C> {
    service: PaymentService<C>,
}

impl <C: PaymentClient> CreatePriceUseCase<C> {
    pub fn new(service: PaymentService<C>) -> Self {
        Self { service }
    }

    pub async fn execute(&self, new_price: NewPriceDto) -> Result<ProductPrice> {
        let result = self.service.search_prices(
            &new_price.currency,
            &new_price.product,
            true
        )
            .await?
            .into_iter()
            .find(|price| price.unit_amount() == new_price.unit_amount);
        if let Some(price) = result {
            return Ok(price);
        }
        self.service.create_price(new_price).await
    }
}

pub struct SearchPricesUseCase<C> {
    service: PaymentService<C>,
}
impl <C: PaymentClient> SearchPricesUseCase<C> {
    pub fn new(service: PaymentService<C>) -> Self {
        Self { service }
    }

    pub async fn execute(&self, query: PriceSearchQuery) -> Result<Vec<ProductPrice>> {
        self.service.search_prices(&query.currency, &query.product, query.active).await
    }
}


//*******************************************************//
//              Create Checkout Use Cases                //
//*******************************************************//
#[derive(Clone)]
pub struct CreateCheckoutSessionUseCase<C> {
    service: PaymentService<C>,
}
impl <C: PaymentClient> CreateCheckoutSessionUseCase<C> {
    pub fn new(service: PaymentService<C>) -> Self {
        Self { service }
    }

    pub async fn execute(&self, new_checkout: NewCheckoutSessionDto) -> Result<CheckoutSession> {
        self.service.create_checkout_session(new_checkout).await
    }
}

#[derive(Clone)]
pub struct CreatePortalSessionUseCase<C> {
    service: PaymentService<C>,
}
impl <C: PaymentClient> CreatePortalSessionUseCase<C> {
    pub fn new(service: PaymentService<C>) -> Self {
        Self { service }
    }

    pub async fn execute(&self, new_portal: NewPortalDto) -> Result<CustomerPortalSession> {
        self.service.create_portal_session(new_portal).await
    }
}
