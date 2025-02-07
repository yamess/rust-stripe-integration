use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::application::user::dtos::{UpdateUserDto, UserDto};
use crate::application::user::extractor::{Authenticate, UserExtractor};
use crate::application::user::use_cases::{DeleteUserUseCase, RegisterNewUserUseCase, UpdateUserUseCase};
use crate::infra::dependencies::AppState;
use crate::prelude::*;

#[post("/register")]
pub async fn register(auth: Authenticate, state: web::Data<AppState>) -> Result<impl Responder> {
    let service = state.user_service.clone();
    let use_case = RegisterNewUserUseCase::new(service);
    match use_case.execute(&auth.0).await {
        Ok(user) => {
            tracing::info!("New user created: {}", user.id);
            Ok(HttpResponse::Ok().json(user))
        },
        Err(e) => Err(e)
    }
}

#[patch("/users")]
pub async fn update_user(
    user: UserExtractor, updates: web::Json<UpdateUserDto>, state: web::Data<AppState>
) -> Result<impl Responder> {
    let user = user.0;
    let updates = updates.into_inner();

    let service = state.user_service.clone();
    let use_case = UpdateUserUseCase::new(service);

    match use_case.execute(updates, &user).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Err(e)
    }
}

#[delete("/users")]
pub async fn delete_user(user: UserExtractor, state: web::Data<AppState>) -> Result<impl Responder> {
    let service = state.user_service.clone();
    let use_case = DeleteUserUseCase::new(service);
    use_case.execute(&user.0).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/users/me")]
pub async fn get_user(user: UserExtractor) -> Result<impl Responder> {
    // This will retrieve the user from the extractor by using auth provider id
    Ok(HttpResponse::Ok().json(user.0))
}

#[get("/users/{id}")]
pub async fn get_user_by_id(user: UserExtractor, user_id: web::Path<Uuid>) -> Result<impl Responder> {
    let user_id = user_id.into_inner();
    if user.0.id == user_id {
        Ok(HttpResponse::Ok().json(user.0))
    } else {
        Err(Error::Unauthorized)
    }
}
