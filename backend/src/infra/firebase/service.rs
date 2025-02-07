use std::sync::Arc;
use crate::domain::user::entities::AuthProviderData;
use crate::domain::user::services::Authenticator;
use crate::prelude::*;
use serde_json::json;
use crate::infra::firebase::model::FirebaseResponse;

#[derive(Debug, Clone)]
pub struct FirebaseAuthenticatorService {
    http: Arc<reqwest::Client>,
    firebase_api_key: String,
}
impl FirebaseAuthenticatorService {
    pub fn new(firebase_api_key: &str, http: Arc<reqwest::Client>) -> Self {
        Self {
            http,
            firebase_api_key: firebase_api_key.to_string(),
        }
    }
}

impl Authenticator for FirebaseAuthenticatorService {
    async fn authenticate(&self, token: &str) -> Result<AuthProviderData> {
       let url = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
            self.firebase_api_key
       );

        let payload = json!({
            "idToken": token
        });

        let response = self.http
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| Error::AuthenticationFailed(e.to_string()))?;

        let response = response
            .json::<FirebaseResponse>()
            .await
            .map_err(|e| Error::DeserializationError(e.to_string()))?;

        if response.users.is_empty() {
            tracing::error!("No users found");
            return Err(Error::AuthenticationFailed("Authentication failed".to_string()));
        }
        if response.users.len() > 1 {
            tracing::error!("Multiple users found");
            return Err(Error::AuthenticationFailed("Authentication failed".to_string()));
        }
        let user = response
            .users
            .first()
            .cloned()
            .ok_or(Error::Unauthorized)?;

        Ok(AuthProviderData::new(user.id, user.email, user.name, user.photo))
    }
}