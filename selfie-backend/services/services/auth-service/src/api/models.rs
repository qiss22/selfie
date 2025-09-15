use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 12, message = "Passphrase must be at least 12 characters"))]
    pub passphrase: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub passphrase: String,
    pub totp_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub totp_enabled: bool,
    pub created_at: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct EnableTotpRequest {
    #[validate(length(equal = 6, message = "TOTP code must be 6 digits"))]
    pub verification_code: String,
}

#[derive(Debug, Serialize)]
pub struct TotpSecretResponse {
    pub secret: String,
    pub qr_code_url: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ConfirmResetRequest {
    pub token: String,
    #[validate(length(min = 12, message = "Passphrase must be at least 12 characters"))]
    pub new_passphrase: String,
}