use crate::infra::dependencies::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/health")]
pub async fn health() -> impl Responder {
    tracing::info!("Health check");
    HttpResponse::Ok().finish()
}

// #[post("/checkout")]
// pub async fn checkout(state: web::Data<AppState>) -> impl Responder {
//     let url = state.payment.create_stripe_session().await;
//     let response = PaymentSessionResponse { url };
//     HttpResponse::Ok().json(response)
// }
