use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Authentication failed")]
    AuthenticationError,
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("User already exists")]
    UserExists,
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] foundationdb::Error),
    
    #[error("Internal server error")]
    InternalError,
    
    #[error("Weak passphrase: {0}")]
    WeakPassphrase(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("2FA is already enabled")]
    TotpAlreadyEnabled,

    #[error("2FA is not enabled")]
    TotpNotEnabled,

    #[error("Invalid TOTP code")]
    InvalidTotpCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::AuthenticationError => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AuthError::UserExists => (StatusCode::CONFLICT, self.to_string()),
            AuthError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred".to_string()),
            AuthError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            AuthError::WeakPassphrase(msg) => (StatusCode::BAD_REQUEST, msg),
            AuthError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, self.to_string()),
        };

        let body = Json(json!({
            "error": message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}