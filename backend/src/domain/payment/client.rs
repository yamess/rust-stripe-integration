use crate::domain::payment::entities::checkout::CheckoutSession;
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::portal::CustomerPortalSession;
use crate::prelude::*;

pub trait PaymentClient: Send + Sync {
    async fn create_customer(&self, customer: &Customer) -> Result<Customer>;
    async fn get_customer(&self, email: &str) -> Result<Customer>;
    async fn create_checkout_session(&self, checkout: &CheckoutSession) -> Result<String>;
    async fn create_portal_session(&self, portal: &CustomerPortalSession) -> Result<String>;
}
