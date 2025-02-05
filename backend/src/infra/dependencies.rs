use crate::domain::entities::Payment;
use crate::infra::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub payment: Payment,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let payment = Payment::new(
            &config.secrets().stripe_secret_key(),
            Some(config.app().success_page.clone()),
            Some(config.app().failure_page.clone()),
        );

        Self {
            config,
            payment,
        }
    }
}