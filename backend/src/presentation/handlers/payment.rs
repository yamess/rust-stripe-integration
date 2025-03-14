use crate::application::payment::dto::{NewCheckoutSessionDto, NewPortalDto};
use crate::application::payment::event_use_cases::UpdateUserEvent;
use crate::application::payment::use_cases::{
    CreateCheckoutSessionUseCase, CreatePortalSessionUseCase,
};
use crate::application::subscription::extractors::SignatureVerifier;
use crate::application::subscription::use_cases::{
    InvoicePaidUseCase, InvoicePaymentFailedUseCase, SubscriptionCanceledUseCase,
    SubscriptionUpdatedUseCase,
};
use crate::application::user::extractor::UserExtractor;
use crate::domain::payment::entities::customer::Customer;
use crate::infra::dependencies::AppState;
use crate::prelude::*;
use actix_web::{post, web, HttpResponse, Responder};
use serde_json::Value;

//
// #[post("/customers")]
// pub async fn create_customer(
//     state: web::Data<AppState>, new_customer: web::Json<NewCustomerDto>
// ) -> Result<impl Responder> {
//     let service = state.payment_service.clone();
//     let use_case = CreateCustomerUseCase::new(service);
//     let new_customer = new_customer.into_inner();
//     match use_case.execute(new_customer).await {
//         Ok(customer) => Ok(HttpResponse::Created().json(customer)),
//         Err(e) => Err(e)
//     }
// }
//
// #[get("/customers/{email}")]
// pub async fn get_customer(
//     state: web::Data<AppState>, email: web::Path<String>
// ) -> Result<impl Responder> {
//     let service = state.payment_service.clone();
//     let use_case = GetCustomerUseCase::new(service);
//     let email = email.into_inner();
//     match use_case.execute(&email).await {
//         Ok(customer) => Ok(HttpResponse::Ok().json(customer)),
//         Err(e) => Err(e)
//     }
// }

#[post("/checkout/sessions")]
pub async fn create_checkout_session(
    user: UserExtractor,
    state: web::Data<AppState>,
    new_checkout: web::Json<NewCheckoutSessionDto>,
) -> Result<impl Responder> {
    let user = user.0;
    let service = state.payment_service.clone();
    let use_case = CreateCheckoutSessionUseCase::new(service);
    let new_checkout = new_checkout.into_inner();
    match use_case.execute(user, new_checkout).await {
        Ok(checkout) => Ok(HttpResponse::Created().json(checkout)),
        Err(e) => Err(e),
    }
}

#[post("/portal/sessions")]
pub async fn create_portal_session(
    user: UserExtractor,
    state: web::Data<AppState>,
    new_portal: web::Json<NewPortalDto>,
) -> Result<impl Responder> {
    let user = user.0;
    let service = state.payment_service.clone();
    let use_case = CreatePortalSessionUseCase::new(service);
    let new_portal = new_portal.into_inner();
    match use_case.execute(user, new_portal).await {
        Ok(portal) => Ok(HttpResponse::Created().json(portal)),
        Err(e) => Err(e),
    }
}

#[post("/webhook")]
pub async fn payment_webhook(
    state: web::Data<AppState>,
    verified: SignatureVerifier<Value>,
) -> Result<impl Responder> {
    let body = &verified.0;
    let event_type = body["type"]
        .as_str()
        .ok_or(Error::BadRequest("Invalid event type".to_string()))?;
    match event_type {
        "customer.created" => {
            tracing::info!("customer.created event received");
            let customer: Customer =
                serde_json::from_value(body["data"]["object"].clone()).unwrap();
            let use_case = UpdateUserEvent::new(state.user_service.clone());
            use_case.execute(customer).await?;
        }
        "invoice.paid" => {
            tracing::info!("invoice.paid event received");
            let data = body["data"]["object"].clone();
            let use_case = InvoicePaidUseCase::new(
                state.subscription_service.clone(),
                state.user_service.clone(),
            );
            use_case.execute(data).await?;
        }
        "invoice.payment_failed" => {
            tracing::info!("invoice.payment_failed event received");
            let data = body["data"]["object"].clone();
            let use_case = InvoicePaymentFailedUseCase::new(
                state.subscription_service.clone(),
                state.user_service.clone(),
            );
            use_case.execute(data).await?;
        }
        "customer.subscription.updated" => {
            tracing::info!("customer.subscription.updated event received");
            let data = body["data"]["object"].clone();
            let use_case = SubscriptionUpdatedUseCase::new(
                state.subscription_service.clone(),
                state.user_service.clone(),
            );
            use_case.execute(data).await?;
        }
        "customer.subscription.deleted" => {
            tracing::info!("customer.subscription.deleted event received");
            let data = body["data"]["object"].clone();
            let use_case = SubscriptionCanceledUseCase::new(
                state.subscription_service.clone(),
                state.user_service.clone(),
            );
            use_case.execute(data).await?;
        }
        _ => {}
    }
    Ok(HttpResponse::Ok().finish())
}
