use crate::application::user::service::UserService;
use crate::domain::entities::Payment;
use crate::infra::config::Config;
use crate::infra::postgres::connection::establish_connection;
use crate::infra::postgres::repositories::PostgresUserRepository;
use std::sync::Arc;
use crate::domain::user::repositories::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub payment: Payment,
    pub user_service: UserService<PostgresUserRepository>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let payment = Payment::new(
            &config.secrets().stripe_secret_key(),
            Some(config.app().success_page.clone()),
            Some(config.app().failure_page.clone()),
        );

        let db_pool = establish_connection(&config.secrets().postgres_connection_string());

        let pg_user_repository = PostgresUserRepository::new(db_pool.clone());

        let user_service = UserService::new(Arc::new(pg_user_repository));
        Self {
            config,
            payment,
            user_service,
        }
    }
}