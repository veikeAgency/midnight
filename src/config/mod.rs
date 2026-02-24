use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub app_env: String,
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub log_level: String,
    pub default_provider: String,
    pub api_key_hash_secret: String,
    pub webhook_signing_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            app_env: env::var("APP_ENV").unwrap_or_else(|_| "development".into()),
            host: env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: env::var("APP_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(8000),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/midnight".into()),
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into()),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into()),
            default_provider: env::var("DEFAULT_PROVIDER").unwrap_or_else(|_| "mock".into()),
            api_key_hash_secret: env::var("API_KEY_HASH_SECRET").unwrap_or_else(|_| "api_key_hash_secret".into()),
            webhook_signing_secret: env::var("WEBHOOK_SIGNING_SECRET").unwrap_or_else(|_| "webhook_signing_secret".into()),
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
