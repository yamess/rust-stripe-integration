use uuid::Uuid;
use crate::domain::payment::old_entities::PaymentSession;
use crate::domain::plans::entities::{Plan, RatePlan};
use crate::domain::user::entities::User;
use crate::prelude::*;

pub trait PaymentService: Send + Sync {
    async fn create_customer(&self, customer: &NewCustomer) -> Result<Customer>;
    async fn create_product(&self, new_product: &NewProduct) -> Result<Product>;
    async fn create_price(&self, new_price: &NewProductPrice) -> Result<ProductPrice>;
    async fn create_checkout_session(
        &self, new_session: &NewCheckoutSession
    ) -> Result<CheckoutSession>;
    // async fn get_plans(&self) -> Result<Vec<Plan>>;
    // async fn create_checkout_session(&self, user_id: Uuid, plan_id: i32) -> Result<String>;
    // async fn upgrade_subscription(&self, user_id: Uuid, plan_id: i32) -> Result<()>;
    // async fn cancel_subscription(&self, user_id: Uuid) -> Result<()>;
}