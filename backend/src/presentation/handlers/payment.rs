use actix_web::{get, post, web, HttpResponse, Responder};
use crate::application::payment::dto::{NewCheckoutSessionDto, NewCustomerDto, NewPortalDto};
use crate::application::payment::use_cases::{CreateCheckoutSessionUseCase, CreateCustomerUseCase,
                                             CreatePortalSessionUseCase,  GetCustomerUseCase};
use crate::application::user::extractor::UserExtractor;
use crate::infra::dependencies::AppState;
use crate::prelude::*;


#[post("/customers")]
pub async fn create_customer(
    state: web::Data<AppState>, new_customer: web::Json<NewCustomerDto>
) -> Result<impl Responder> {
    let service = state.payment_service.clone();
    let use_case = CreateCustomerUseCase::new(service);
    let new_customer = new_customer.into_inner();
    match use_case.execute(new_customer).await {
        Ok(customer) => Ok(HttpResponse::Created().json(customer)),
        Err(e) => Err(e)
    }
}

#[get("/customers/{email}")]
pub async fn get_customer(
    state: web::Data<AppState>, email: web::Path<String>
) -> Result<impl Responder> {
    let service = state.payment_service.clone();
    let use_case = GetCustomerUseCase::new(service);
    let email = email.into_inner();
    match use_case.execute(&email).await {
        Ok(customer) => Ok(HttpResponse::Ok().json(customer)),
        Err(e) => Err(e)
    }
}

#[post("/checkout/sessions")]
pub async fn create_checkout_session(
    user: UserExtractor,
    state: web::Data<AppState>,
    new_checkout: web::Json<NewCheckoutSessionDto>
) -> Result<impl Responder> {
    let user = user.0;
    let service = state.payment_service.clone();
    let use_case = CreateCheckoutSessionUseCase::new(service);
    let new_checkout = new_checkout.into_inner();
    match use_case.execute(user, new_checkout).await {
        Ok(checkout) => Ok(HttpResponse::Created().json(checkout)),
        Err(e) => Err(e)
    }
}

#[post("/portal/sessions")]
pub async fn create_portal_session(
    user: UserExtractor,
    state: web::Data<AppState>,
    new_portal: web::Json<NewPortalDto>
) -> Result<impl Responder> {
    let user = user.0;
    let service = state.payment_service.clone();
    let use_case = CreatePortalSessionUseCase::new(service);
    let new_portal = new_portal.into_inner();
    match use_case.execute(user, new_portal).await {
        Ok(portal) => Ok(HttpResponse::Created().json(portal)),
        Err(e) => Err(e)
    }
}