use crate::presentation::handlers::users;

pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(users::register)
        .service(users::get_user)
        .service(users::get_user_by_id)
        .service(users::update_user)
        .service(users::delete_user);
}