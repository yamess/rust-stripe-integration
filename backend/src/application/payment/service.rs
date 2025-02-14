use std::sync::Arc;
use crate::application::payment::dto::{NewCheckoutSessionDto, NewCustomerDto, NewPortalDto,SessionDto};
use crate::application::user::dtos::UserDto;
use crate::domain::payment::client::PaymentClient;
use crate::domain::payment::entities::checkout::{CheckoutSession};
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::portal::CustomerPortalSession;
use crate::domain::user::entities::User;
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

    pub async fn create_checkout_session(
        &self, checkout: CheckoutSession
    ) -> Result<SessionDto> {
        let result = self.client.create_checkout_session(&checkout).await?;
        Ok(SessionDto::new(result))
    }

    pub async fn create_portal_session(
        &self, portal: CustomerPortalSession
    ) -> Result<SessionDto> {
        // let portal = CustomerPortalSession::try_from(new_portal)?;
        let result = self.client.create_portal_session(&portal).await?;
        Ok(SessionDto::new(result))
    }
}
