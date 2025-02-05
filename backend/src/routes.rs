use crate::handlers::{health, checkout};

pub fn probe_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(health).service(checkout);
}