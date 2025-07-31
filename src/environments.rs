use std::env;

/// Initialize environment variables from a `.env` file if it exists.
pub fn init_env() {
    dotenvy::dotenv()
        .map_err(|e| tracing::warn!("Failed to load .env file: {}", e))
        .ok();
}

/// Get the development environment flag. True if the application is running in development mode.
pub fn is_development() -> bool {
    env::var("RUST_LOG").map_or(false, |log| log.contains("debug"))
}

/// Get the JWT secret from environment variables.
pub fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

/// Get the JWT expiration time in days from environment variables.
pub fn get_jwt_expiration_days() -> i64 {
    env::var("JWT_EXPIRATION_DAYS")
        .map(|val| val.parse().unwrap_or(15)) // Default to 15 days if not set
        .unwrap_or(15)
}
