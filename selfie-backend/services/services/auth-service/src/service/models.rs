use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub passphrase_hash: String,
    pub totp_secret: Option<String>,
    pub totp_enabled: bool,
    pub failed_login_attempts: u32,
    pub last_login: Option<OffsetDateTime>,
    pub email_verified: bool,
    pub email_verification_token: Option<String>,
    pub password_reset_token: Option<String>,
    pub password_reset_expires: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub status: UserStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    PendingVerification,
}

#[derive(Debug)]
pub struct AuthToken {
    pub user_id: Uuid,
    pub token_type: TokenType,
    pub expires_at: OffsetDateTime,
}

#[derive(Debug)]
pub enum TokenType {
    Access,
    Refresh,
    EmailVerification,
    PasswordReset,
}

#[derive(Debug)]
pub struct TotpSetup {
    pub secret: String,
    pub provisioning_uri: String,
}

impl User {
    pub fn new(email: String, passphrase_hash: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            email,
            passphrase_hash,
            totp_secret: None,
            totp_enabled: false,
            failed_login_attempts: 0,
            last_login: None,
            email_verified: false,
            email_verification_token: None,
            password_reset_token: None,
            password_reset_expires: None,
            created_at: now,
            updated_at: now,
            status: UserStatus::PendingVerification,
        }
    }

    pub fn is_locked(&self) -> bool {
        const MAX_ATTEMPTS: u32 = 5;
        const LOCKOUT_DURATION: i64 = 3600; // 1 hour in seconds

        if self.failed_login_attempts >= MAX_ATTEMPTS {
            if let Some(last_login) = self.last_login {
                return last_login + time::Duration::seconds(LOCKOUT_DURATION) > OffsetDateTime::now_utc();
            }
        }
        false
    }
}