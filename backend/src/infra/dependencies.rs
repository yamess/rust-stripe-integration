use crate::application::payment::service::PaymentService;
use crate::application::subscription::service::{SignatureService, SubscriptionService};
use crate::application::user::service::{AuthenticationService, UserService};
use crate::infra::config::Config;
use crate::infra::firebase::service::FirebaseAuthenticatorService;
use crate::infra::postgres::connection::establish_connection;
use crate::infra::postgres::migrations::run_migrations;
use crate::infra::postgres::repositories::subscription::PostgresSubscriptionRepository;
use crate::infra::postgres::repositories::user::PostgresUserRepository;
use crate::infra::stripe::payment::StripePaymentClient;
use crate::infra::stripe::service::StripeSignatureVerificationService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub user_service: UserService<PostgresUserRepository>,
    pub auth_service: AuthenticationService<FirebaseAuthenticatorService>,
    pub payment_service: PaymentService<StripePaymentClient>,
    pub subscription_service: SubscriptionService<PostgresSubscriptionRepository>,
    pub signature_service: SignatureService<StripeSignatureVerificationService>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let db_pool = establish_connection(&config.secrets().postgres_connection_string());

        run_migrations(db_pool.clone()).unwrap();

        let http_client = Arc::new(reqwest::Client::new());

        let pg_user_repository = Arc::new(PostgresUserRepository::new(db_pool.clone()));
        let auth_client = Arc::new(FirebaseAuthenticatorService::new(
            config.secrets().firebase_api_key(),
            http_client.clone(),
        ));
        let payment_client = Arc::new(StripePaymentClient::new(
            config.secrets().stripe_secret_key(),
            http_client.clone(),
            "https://api.stripe.com/v1",
        ));
        let subscription_repository =
            Arc::new(PostgresSubscriptionRepository::new(db_pool.clone()));
        let stripe_signature_service = Arc::new(StripeSignatureVerificationService::new(
            config.secrets().stripe_webhook_secret(),
        ));

        let user_service = UserService::new(pg_user_repository);
        let auth_service = AuthenticationService::new(auth_client);
        let payment_service = PaymentService::new(payment_client);
        let subscription_service = SubscriptionService::new(subscription_repository);
        let signature_service = SignatureService::new(stripe_signature_service);
        Self {
            config,
            user_service,
            auth_service,
            payment_service,
            subscription_service,
            signature_service,
        }
    }
}
