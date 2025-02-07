use crate::presentation::handlers::probes::health;

pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(health);
        // .service(checkout);
}
