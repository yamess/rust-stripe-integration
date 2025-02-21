use crate::infra::dependencies::AppState;
use crate::prelude::*;
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{web, FromRequest, HttpMessage, HttpRequest};
use futures_util::future::LocalBoxFuture;
use futures_util::StreamExt;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct SignatureVerifier<T>(pub T);

impl<T: DeserializeOwned + 'static> FromRequest for SignatureVerifier<T> {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let signature = req
            .headers()
            .get("Stripe-Signature")
            .and_then(|value| value.to_str().ok())
            .map(String::from);

        let state = req.app_data::<Data<AppState>>().cloned();

        // let mut payload = payload.take();
        let payload_future = web::Bytes::from_request(req, payload);

        Box::pin(async move {
            let signature = signature.ok_or_else(|| {
                tracing::error!("Missing Stripe-Signature header");
                Error::Unauthorized
            })?;
            let state = state.ok_or_else(|| {
                tracing::error!("App state not found");
                Error::InternalError
            })?;
            let raw_body = payload_future.await.map_err(|e| {
                tracing::error!("Failed to read request body: {}", e);
                Error::InternalError
            })?;
            // Convert raw body to string for verification
            let payload_str = String::from_utf8(raw_body.to_vec()).map_err(|e| {
                tracing::error!("Invalid UTF-8 in request body: {}", e);
                Error::InternalError
            })?;
            // Verify signature
            state.signature_service.verify(&payload_str, &signature)?;

            let json_payload: T = serde_json::from_slice(&raw_body).map_err(|e| {
                tracing::error!("Failed to parse request body: {}", e);
                Error::InternalError
            })?;
            Ok(SignatureVerifier(json_payload))
        })
    }
}
