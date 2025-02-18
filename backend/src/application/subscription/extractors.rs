use actix_web::{FromRequest, HttpMessage, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::Data;
use futures_util::future::LocalBoxFuture;
use futures_util::StreamExt;
use crate::infra::dependencies::AppState;
use crate::infra::stripe::service::StripeSignatureVerificationService;
use crate::prelude::*;


#[derive(Debug)]
pub struct SignatureVerifier(());

impl FromRequest for SignatureVerifier {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {


        let signature = req
            .headers()
            .get("Stripe-Signature")
            .and_then(|value| value.to_str().ok())
            .map(String::from);
        let state = req.app_data::<Data<AppState>>().cloned();
        let mut payload = payload.take();

        Box::pin(async move {

            if let (Some(signature), Some(state)) = (signature, state) {
                if state.config.app().environment != "production" {
                    return Ok(SignatureVerifier(()));
                }
                let signature_service = state.signature_service.clone();

                let mut body_bytes = Vec::new();

                while let Some(chunk) = payload.next().await {
                    let chunk = chunk.map_err(|e| {
                        tracing::error!("Failed to read request body: {}", e);
                        Error::InternalError
                    })?;
                    body_bytes.extend_from_slice(&chunk);
                }

                let body = String::from_utf8(body_bytes).map_err(|e| {
                    tracing::error!("Failed to parse request body: {}", e);
                    Error::InternalError
                })?;

                match signature_service.verify(&body, &signature) {
                    Ok(()) => Ok(SignatureVerifier(())),
                    Err(e) => {
                        tracing::error!("Invalid signature: {}", e);
                        Err(Error::Unauthorized)
                    }
                }
            } else {
                tracing::error!("Unauthorized");
                Err(Error::Unauthorized)
            }
        })
    }
}