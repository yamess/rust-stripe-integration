use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct FirebaseUser {
    #[serde(alias = "localId")]
    pub id: String,
    pub email: String,
    #[serde(alias = "emailVerified")]
    pub email_verified: bool,
    #[serde(alias = "displayName")]
    pub name: Option<String>,
    #[serde(alias = "photoUrl")]
    pub photo: Option<String>,
    #[serde(alias = "validSince")]
    pub valid_since: String,
    #[serde(alias = "lastLoginAt")]
    pub last_login_at: String,
    #[serde(alias = "createdAt")]
    pub created_at: String,
    #[serde(alias = "lastRefreshAt")]
    pub last_refresh_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirebaseResponse {
    pub users: Vec<FirebaseUser>,
}
