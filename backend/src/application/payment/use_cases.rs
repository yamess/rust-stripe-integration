use crate::application::payment::dto::{NewCheckoutSessionDto, NewCustomerDto, NewPortalDto, NewPriceDto, NewProductDto};
use crate::application::payment::service::PaymentService;
use crate::domain::payment::client::PaymentClient;
use crate::domain::payment::entities::checkout::CheckoutSession;
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::portal::CustomerPortalSession;
use crate::domain::payment::entities::product::Product;
use crate::domain::payment::entities::product_price::ProductPrice;
use crate::prelude::*;


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

#[derive(Clone)]
pub struct CreateProductUseCase<C> {
    service: PaymentService<C>,
}
impl <C: PaymentClient> CreateProductUseCase<C> {
    pub fn new(service: PaymentService<C>) -> Self {
        Self { service }
    }

    pub async fn execute(&self, new_product: NewProductDto) -> Result<Product> {
        self.service.create_product(new_product).await
    }
}

#[derive(Clone)]
pub struct CreatePriceUseCase<C> {
    service: PaymentService<C>,
}
impl <C: PaymentClient> CreatePriceUseCase<C> {
    pub fn new(service: PaymentService<C>) -> Self {
        Self { service }
    }

    pub async fn execute(&self, new_price: NewPriceDto) -> Result<ProductPrice> {
        self.service.create_price(new_price).await
    }
}

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
