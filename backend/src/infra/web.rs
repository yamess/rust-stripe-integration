use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use actix_web::web::{scope, Data};
use clap::Parser;
use crate::infra::cli::Args;
use crate::infra::config::Config;
use crate::infra::dependencies::AppState;
use crate::presentation::routers;


pub async fn run() -> std::io::Result<()> {
    let args = Args::parse();

    let config = Config::new(&args.config_path, &args.secret_path);

    // Initialize logger
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(&config.app().log_level),
    )
    .init();

    let app_state = AppState::new(config.clone());


    let cors_origin = config.app().cors_origin.clone();

    HttpServer::new(move || {

        let cors = Cors::default()
            .allowed_origin(&cors_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Authorization", "Content-Type"])
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .app_data(Data::new(app_state.clone()))
            .service(
                scope("/v1/payment")
                    .configure(routers::payment::routes)
            )
            .service(
                scope("/v1")
                    .configure(routers::probes::routes)
                    .configure(routers::users::routes)
            )
    })
    .bind(format!("{}:{}", &config.app().host, &config.app().port))?
    .run()
    .await?;

    Ok(())
}