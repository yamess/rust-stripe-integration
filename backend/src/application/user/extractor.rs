use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::Data;
use futures_util::future::LocalBoxFuture;
use crate::application::user::dtos::UserDto;
use crate::application::user::use_cases::GetUserByAuthProviderIdUseCase;
use crate::domain::user::entities::AuthProviderData;
use crate::infra::dependencies::AppState;
use crate::prelude::*;


#[derive(Debug, Clone)]
pub struct Authenticate(pub AuthProviderData);

impl FromRequest for Authenticate {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        tracing::info!("Extracting user from request");
        let bearer_token = req
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .map(|value| value.replace("Bearer ", ""))
            .map(String::from);

        let app_state = req.app_data::<Data<AppState>>().cloned();

        Box::pin(async move {
            tracing::debug!("Inside the Box::pin block");

            if let (Some(token), Some(state)) = (bearer_token, app_state) {
                let auth_service = state.auth_service.clone();
                match auth_service.authenticate(&token).await {
                    Ok(auth) => Ok(Authenticate(auth)),
                    Err(e) => {
                        tracing::error!("Invalid credentials: {}", e);
                        Err(Error::InvalidToken(e.to_string()))
                    }
                }
            } else {
                tracing::error!("Unauthorized");
                Err(Error::Unauthorized)
            }
        })
    }
}

pub struct UserExtractor(pub UserDto);

impl FromRequest for UserExtractor {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        tracing::info!("Extracting user from request");
        let bearer_token = req
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .map(|value| value.replace("Bearer ", ""))
            .map(String::from);

        let app_state = req.app_data::<Data<AppState>>().cloned();

        Box::pin(async move {
            tracing::debug!("Inside the Box::pin block");

            if let (Some(token), Some(state)) = (bearer_token, app_state) {
                let auth_service = state.auth_service.clone();
                let user_service = state.user_service.clone();
                let use_case = GetUserByAuthProviderIdUseCase::new(user_service);

                match auth_service.authenticate(&token).await {
                    Ok(auth) => {
                        let user = use_case.execute(&auth.id).await?;
                        Ok(UserExtractor(user))
                    }
                    Err(e) => {
                        tracing::error!("Invalid credentials: {}", e);
                        Err(Error::InvalidToken(e.to_string()))
                    }
                }
            } else {
                tracing::error!("Unauthorized");
                Err(Error::Unauthorized)
            }
        })
    }
}
