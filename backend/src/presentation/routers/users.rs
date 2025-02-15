use crate::presentation::handlers::users;

pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(users::login)
        .service(users::get_user)
        .service(users::update_user)
        .service(users::delete_user);
}