use crate::application::user::service::{AuthenticationService, UserService};
use crate::domain::entities::Payment;
use crate::infra::config::Config;
use crate::infra::postgres::connection::establish_connection;
use std::sync::Arc;
use crate::domain::user::repositories::UserRepository;
use crate::infra::firebase::service::FirebaseAuthenticatorService;
use crate::infra::postgres::repositories::user::PostgresUserRepository;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub payment: Payment,
    pub user_service: UserService<PostgresUserRepository>,
    pub auth_service: AuthenticationService<FirebaseAuthenticatorService>
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let payment = Payment::new(
            &config.secrets().stripe_secret_key(),
            Some(config.app().success_page.clone()),
            Some(config.app().failure_page.clone()),
        );

        let db_pool = establish_connection(&config.secrets().postgres_connection_string());
        let http_client = Arc::new(reqwest::Client::new());

        let pg_user_repository = Arc::new(PostgresUserRepository::new(db_pool.clone()));
        let auth_client = Arc::new(FirebaseAuthenticatorService::new(
            config.secrets().firebase_api_key(), http_client.clone()
        ));

        let user_service = UserService::new(pg_user_repository);
        let auth_service = AuthenticationService::new(auth_client);
        Self {
            config,
            payment,
            user_service,
            auth_service
        }
    }
}