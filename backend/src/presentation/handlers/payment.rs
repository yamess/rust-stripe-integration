use actix_web::{get, post, web, HttpResponse, Responder};
use crate::application::payment::dto::{NewCheckoutSessionDto, NewCustomerDto, NewPortalDto, NewPriceDto, NewProductDto, PriceSearchQuery};
use crate::application::payment::use_cases::{CreateCheckoutSessionUseCase, CreateCustomerUseCase,
                                             CreatePortalSessionUseCase, CreatePriceUseCase, CreateProductUseCase, GetCustomerUseCase, GetProductUseCase, SearchPricesUseCase};
use crate::domain::plans::value_objects::currency::Currency;
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

#[post("/products")]
pub async fn create_product(
    state: web::Data<AppState>, new_product: web::Json<NewProductDto>
) -> Result<impl Responder> {
    let service = state.payment_service.clone();
    let use_case = CreateProductUseCase::new(service);
    let new_product = new_product.into_inner();
    match use_case.execute(new_product).await {
        Ok(product) => Ok(HttpResponse::Created().json(product)),
        Err(e) => Err(e)
    }
}
#[get("/products/{name}")]
pub async fn get_product(
    state: web::Data<AppState>, name: web::Path<String>
) -> Result<impl Responder> {
    let service = state.payment_service.clone();
    let use_case = GetProductUseCase::new(service);
    let name = name.into_inner();
    match use_case.execute(&name).await {
        Ok(product) => Ok(HttpResponse::Ok().json(product)),
        Err(e) => Err(e)
    }
}

#[post("/prices")]
pub async fn create_price(
    state: web::Data<AppState>, new_price: web::Json<NewPriceDto>
) -> Result<impl Responder> {
    let service = state.payment_service.clone();
    let use_case = CreatePriceUseCase::new(service);
    let new_price = new_price.into_inner();
    match use_case.execute(new_price).await {
        Ok(price) => Ok(HttpResponse::Created().json(price)),
        Err(e) => Err(e)
    }
}

#[get("/prices")]
pub async fn search_prices(
    state: web::Data<AppState>, query: web::Query<PriceSearchQuery>
) -> Result<impl Responder> {
    let service = state.payment_service.clone();
    let use_case = SearchPricesUseCase::new(service);
    let query = query.into_inner();
    match use_case.execute(query).await {
        Ok(prices) => Ok(HttpResponse::Ok().json(prices)),
        Err(e) => Err(e)
    }
}


#[post("/checkout/sessions")]
pub async fn create_checkout_session(
    state: web::Data<AppState>, new_checkout: web::Json<NewCheckoutSessionDto>
) -> Result<impl Responder> {
    let service = state.payment_service.clone();
    let use_case = CreateCheckoutSessionUseCase::new(service);
    let new_checkout = new_checkout.into_inner();
    match use_case.execute(new_checkout).await {
        Ok(checkout) => Ok(HttpResponse::Created().json(checkout)),
        Err(e) => Err(e)
    }
}

#[post("/portal/sessions")]
pub async fn create_portal_session(
    state: web::Data<AppState>, new_portal: web::Json<NewPortalDto>
) -> Result<impl Responder> {
    let service = state.payment_service.clone();
    let use_case = CreatePortalSessionUseCase::new(service);
    let new_portal = new_portal.into_inner();
    match use_case.execute(new_portal).await {
        Ok(portal) => Ok(HttpResponse::Created().json(portal)),
        Err(e) => Err(e)
    }
}