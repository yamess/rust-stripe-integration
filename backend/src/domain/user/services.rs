use crate::domain::user::entities::AuthProviderData;
use crate::prelude::*;

pub trait Authenticator: Send + Sync {
    async fn authenticate(&self, token: &str) -> Result<AuthProviderData>;
}
