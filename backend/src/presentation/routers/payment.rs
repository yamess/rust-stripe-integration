use crate::presentation::handlers::payment;

pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(payment::create_customer)
        .service(payment::get_customer)
        .service(payment::create_checkout_session)
        .service(payment::create_portal_session);
}