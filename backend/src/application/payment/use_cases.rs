use crate::application::payment::dto::{NewCheckoutSessionDto, NewCustomerDto, NewPortalDto,SessionDto};
use crate::application::payment::service::PaymentService;
use crate::application::user::dtos::UserDto;
use crate::domain::payment::client::PaymentClient;
use crate::domain::payment::entities::checkout::CheckoutSession;
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::portal::CustomerPortalSession;
use crate::domain::user::entities::User;
use crate::prelude::*;

// //************************************************//
// //            Create Customer Use Cases                   //
// //************************************************//
// #[derive(Clone)]
// pub struct CreateCustomerUseCase<C> {
//     service: PaymentService<C>,
// }
// impl <C: PaymentClient> CreateCustomerUseCase<C> {
//     pub fn new(service: PaymentService<C>) -> Self {
//         Self { service }
//     }
//
//     pub async fn execute(&self, new_customer: NewCustomerDto) -> Result<Customer> {
//         let result = self.service.get_customer(&new_customer.email).await;
//         match result {
//             Ok(customer) => Ok(customer),
//             Err(Error::NotFound(_)) => self.service.create_customer(new_customer).await,
//             Err(e) => Err(e),
//         }
//
//     }
// }
//
// pub struct GetCustomerUseCase<C> {
//     service: PaymentService<C>,
// }
// impl <C: PaymentClient> GetCustomerUseCase<C> {
//     pub fn new(service: PaymentService<C>) -> Self {
//         Self { service }
//     }
//
//     pub async fn execute(&self, email: &str) -> Result<Customer> {
//         self.service.get_customer(email).await
//     }
// }
//
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

    pub async fn execute(
        &self, user: UserDto, new_checkout: NewCheckoutSessionDto
    ) -> Result<SessionDto> {
        let user = User::try_from(&user)?;
        match user.stripe_customer_id() {
            None => {
                tracing::debug!("Creating new customer for user: {}", &user.id());
                let customer = match self.service.get_customer(user.email()).await {
                    Err(Error::NotFound(_)) => {
                        let new_customer = NewCustomerDto {
                            email: user.email().to_string(),
                            name: user.profile().full_name(),
                        };
                        self.service.create_customer(new_customer).await?
                    },
                    Err(_) => return Err(Error::BadRequest("Failed to create customer".to_string())),
                    Ok(customer) => customer,
                };

                tracing::debug!("Customer created: {:?}", &customer);
                let checkout_session = CheckoutSession::new(
                    customer.id().to_string(),
                    new_checkout.line_items,
                    new_checkout.success_url,
                    new_checkout.cancel_url,
                );
                tracing::info!("Creating checkout session for user: {:?}", &user.id());
                let session = self.service.create_checkout_session(checkout_session).await?;
                Ok(session)
            },
            Some(id) => {
                tracing::debug!("Customer already exists for user: {}", &user.id());
                let checkout_session = CheckoutSession::new(
                    id.to_string(),
                    new_checkout.line_items,
                    new_checkout.success_url,
                    new_checkout.cancel_url,
                );
                tracing::info!("Creating checkout session for user: {:?}", &user.id());
                let session = self.service.create_checkout_session(checkout_session).await?;
                Ok(session)
            }
        }
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

    pub async fn execute(&self, user: UserDto, new_portal: NewPortalDto) -> Result<SessionDto> {
        let user = User::try_from(&user)?;
        match user.stripe_customer_id() {
            None => {
                tracing::error!("User does not have a stripe customer id");
                Err(Error::BadRequest("User does not have a stripe customer id".to_string()))
            },
            Some(id) => {
                let portal = CustomerPortalSession::new(
                    id.to_string(),
                    new_portal.return_url,
                );
                self.service.create_portal_session(portal).await
            }
        }
    }
}
