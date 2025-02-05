use std::path::{Path, PathBuf};
use serde::Deserialize;
use crate::prelude::*;


#[derive(Debug, Clone, Deserialize)]
pub struct Secrets {
    stripe_secret_key: String,
}
impl Secrets {
    pub fn new<P: AsRef<Path>>(secrets_path: P) -> Self {
        let base_path = PathBuf::from(secrets_path.as_ref());

        let stripe_secret_key = Self::read_secret_file(&base_path.join("stripe-secret-key")).unwrap();

        Self {
            stripe_secret_key,
        }
    }
    pub fn read_secret_file(path: &Path) -> Result<String> {
        if !path.exists() {
            return Err(Error::SecretNotFound(path.to_str().unwrap().to_string()));
        }
        let secret_str = std::fs::read_to_string(path).unwrap();
        Ok(secret_str.trim().to_string())
    }
    pub fn stripe_secret_key(&self) -> &str {
        &self.stripe_secret_key
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub cors_origin: String,
    pub success_page: String,
    pub failure_page: String,
}
impl AppConfig {
    pub fn new(config_str: &str) -> Self {
        toml::from_str::<Self>(config_str).unwrap()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    app: AppConfig,
    secrets: Secrets,
}
impl Config {
    pub fn new(config_path: &str, secret_path: &str) -> Self {
        let config_str = std::fs::read_to_string(config_path).unwrap();
        Self {
            app: AppConfig::new(&config_str),
            secrets: Secrets::new(secret_path),
        }
    }

    pub fn app(&self) -> &AppConfig {
        &self.app
    }
    pub fn secrets(&self) -> &Secrets {
        &self.secrets
    }
}