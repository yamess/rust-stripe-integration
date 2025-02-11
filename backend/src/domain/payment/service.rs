use crate::domain::payment::entities::checkout::CheckoutSession;
use crate::domain::payment::entities::customer::Customer;
use crate::domain::payment::entities::portal::CustomerPortalSession;
use crate::domain::payment::entities::product_price::ProductPrice;
use crate::domain::payment::entities::product::Product;
use crate::prelude::*;

pub trait PaymentClient: Send + Sync {
    async fn create_customer(&self, customer: &Customer) -> Result<Customer>;
    async fn create_product(&self, product: &Product) -> Result<Product>;
    async fn create_price(&self, price: &ProductPrice) -> Result<ProductPrice>;
    async fn create_checkout_session(&self, checkout: &CheckoutSession) -> Result<CheckoutSession>;
    async fn create_billing_portal_session(
        &self, portal: &CustomerPortalSession
    ) -> Result<CustomerPortalSession>;
    // async fn get_plans(&self) -> Result<Vec<Plan>>;
    // async fn create_checkout_session(&self, user_id: Uuid, plan_id: i32) -> Result<String>;
    // async fn upgrade_subscription(&self, user_id: Uuid, plan_id: i32) -> Result<()>;
    // async fn cancel_subscription(&self, user_id: Uuid) -> Result<()>;
}