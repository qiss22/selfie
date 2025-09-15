use std::sync::Arc;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use time::OffsetDateTime;
use uuid::Uuid;
use zxcvbn::zxcvbn;

use crate::{
    api::models::{AuthResponse, LoginRequest, RegisterRequest, TotpSecretResponse, EnableTotpRequest},
    error::AuthError,
    repository::UserRepository,
    service::{jwt::JwtService, totp::TotpService, models::{User, UserStatus}},
};

const MIN_ENTROPY_BITS: f64 = 50.0; // Requires a strong passphrase

pub struct AuthService {
    repository: Arc<dyn UserRepository>,
    jwt_service: Arc<JwtService>,
    totp_service: TotpService,
    email_service: Arc<EmailService>,
    argon2: Argon2<'static>,
}

impl AuthService {
    pub fn new(
        repository: Arc<dyn UserRepository>,
        jwt_service: Arc<JwtService>,
        email_service: Arc<EmailService>,
    ) -> Self {
        let argon2 = Argon2::default();
        let totp_service = TotpService::new();
        Self {
            repository,
            jwt_service,
            totp_service,
            email_service,
            argon2,
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<User, AuthError> {
        // Validate passphrase strength
        let entropy = zxcvbn(&req.passphrase, &[&req.email])
            .map_err(|_| AuthError::InternalError)?;

        if entropy.score() < 3 || entropy.entropy_bits() < MIN_ENTROPY_BITS {
            return Err(AuthError::WeakPassphrase(
                "Passphrase is too weak. Please use a longer, more complex passphrase.".to_string(),
            ));
        }

        // Hash the passphrase using Argon2id
        let salt = SaltString::generate(&mut OsRng);
        let passphrase_hash = self
            .argon2
            .hash_password(req.passphrase.as_bytes(), &salt)
            .map_err(|_| AuthError::InternalError)?
            .to_string();

        let user = User::new(req.email, passphrase_hash);
        self.repository.create_user(&user).await?;

        Ok(user)
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, AuthError> {
        let user = self
            .repository
            .get_user_by_email(&req.email)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        if user.is_locked() {
            return Err(AuthError::RateLimitExceeded);
        }

        // Verify passphrase
        let parsed_hash = PasswordHash::new(&user.passphrase_hash)
            .map_err(|_| AuthError::InternalError)?;

        if self
            .argon2
            .verify_password(req.passphrase.as_bytes(), &parsed_hash)
            .is_err()
        {
            // Update failed login attempts
            let mut user = user;
            user.failed_login_attempts += 1;
            user.updated_at = OffsetDateTime::now_utc();
            self.repository.update_user(&user).await?;

            return Err(AuthError::InvalidCredentials);
        }

        // Verify TOTP if enabled
        if user.totp_enabled {
            let totp_code = req.totp_code.ok_or(AuthError::AuthenticationError)?;
            self.verify_totp(&user, &totp_code).await?;
        }

        // Generate tokens
        let access_token = self.jwt_service.generate_access_token(user.id)?;
        let refresh_token = self.jwt_service.generate_refresh_token(user.id)?;

        // Reset failed attempts and update last login
        let mut user = user;
        user.failed_login_attempts = 0;
        user.last_login = Some(OffsetDateTime::now_utc());
        user.updated_at = OffsetDateTime::now_utc();
        self.repository.update_user(&user).await?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 900, // 15 minutes in seconds
        })
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AuthResponse, AuthError> {
        let user_id = self.jwt_service.verify_token(refresh_token, "refresh")?;
        
        // Generate new tokens
        let access_token = self.jwt_service.generate_access_token(user_id)?;
        let new_refresh_token = self.jwt_service.generate_refresh_token(user_id)?;

        Ok(AuthResponse {
            access_token,
            refresh_token: new_refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 900,
        })
    }

    pub async fn setup_totp(&self, user_id: Uuid) -> Result<TotpSecretResponse, AuthError> {
        let mut user = self.repository
            .get_user_by_id(&user_id)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        // Generate new TOTP secret if not already set up
        if user.totp_secret.is_some() {
            return Err(AuthError::InternalError);
        }

        let secret = self.totp_service.generate_secret()?;
        let qr_code_url = self.totp_service.generate_provisioning_uri(&secret, &user.email)?;

        // Store secret but don't enable 2FA yet - user needs to verify first
        user.totp_secret = Some(secret.clone());
        user.updated_at = OffsetDateTime::now_utc();
        self.repository.update_user(&user).await?;

        Ok(TotpSecretResponse {
            secret,
            qr_code_url,
        })
    }

    pub async fn enable_totp(&self, user_id: Uuid, req: EnableTotpRequest) -> Result<(), AuthError> {
        let mut user = self.repository
            .get_user_by_id(&user_id)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        let secret = user.totp_secret
            .as_ref()
            .ok_or(AuthError::AuthenticationError)?;

        // Verify the provided code
        if !self.totp_service.verify_code(secret, &req.verification_code)? {
            return Err(AuthError::InvalidCredentials);
        }

        // Enable 2FA
        user.totp_enabled = true;
        user.updated_at = OffsetDateTime::now_utc();
        self.repository.update_user(&user).await?;

        Ok(())
    }

    pub async fn disable_totp(&self, user_id: Uuid, verification_code: &str) -> Result<(), AuthError> {
        let mut user = self.repository
            .get_user_by_id(&user_id)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        let secret = user.totp_secret
            .as_ref()
            .ok_or(AuthError::AuthenticationError)?;

        // Verify the provided code one last time
        if !self.totp_service.verify_code(secret, verification_code)? {
            return Err(AuthError::InvalidCredentials);
        }

        // Disable 2FA and remove secret
        user.totp_enabled = false;
        user.totp_secret = None;
        user.updated_at = OffsetDateTime::now_utc();
        self.repository.update_user(&user).await?;

        Ok(())
    }

    pub async fn verify_totp(&self, user: &User, code: &str) -> Result<(), AuthError> {
        let secret = user.totp_secret
            .as_ref()
            .ok_or(AuthError::AuthenticationError)?;

        if !self.totp_service.verify_code(secret, code)? {
            return Err(AuthError::InvalidCredentials);
        }

        Ok(())
    }

    pub async fn send_verification_email(&self, user: &User) -> Result<(), AuthError> {
        let token = base64::encode(self.jwt_service.sign_data(user.id.as_bytes()));
        let mut user = user.clone();
        user.email_verification_token = Some(token.clone());
        self.repository.update_user(&user).await?;
        self.email_service.send_verification_email(&user.email, &token).await
    }

    pub async fn verify_email(&self, token: &str) -> Result<(), AuthError> {
        let user = self.repository
            .get_user_by_verification_token(token)
            .await?
            .ok_or(AuthError::InvalidToken)?;

        if user.email_verified {
            return Ok(());
        }

        let mut user = user;
        user.email_verified = true;
        user.email_verification_token = None;
        user.status = UserStatus::Active;
        user.updated_at = OffsetDateTime::now_utc();
        
        self.repository.update_user(&user).await
    }

    pub async fn initiate_password_reset(&self, email: &str) -> Result<(), AuthError> {
        let user = self.repository
            .get_user_by_email(email)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        if !user.email_verified {
            return Err(AuthError::AuthenticationError);
        }

        let token = base64::encode(self.jwt_service.sign_data(user.id.as_bytes()));
        let mut user = user;
        user.password_reset_token = Some(token.clone());
        user.password_reset_expires = Some(OffsetDateTime::now_utc() + time::Duration::hours(1));
        user.updated_at = OffsetDateTime::now_utc();
        
        self.repository.update_user(&user).await?;
        self.email_service.send_password_reset_email(&user.email, &token).await
    }

    pub async fn reset_password(&self, token: &str, new_passphrase: &str) -> Result<(), AuthError> {
        let user = self.repository
            .get_user_by_reset_token(token)
            .await?
            .ok_or(AuthError::InvalidToken)?;

        // Check if token is expired
        if let Some(expires) = user.password_reset_expires {
            if expires < OffsetDateTime::now_utc() {
                return Err(AuthError::TokenExpired);
            }
        } else {
            return Err(AuthError::InvalidToken);
        }

        // Validate new passphrase strength
        let entropy = zxcvbn(new_passphrase, &[&user.email])
            .map_err(|_| AuthError::InternalError)?;

        if entropy.score() < 3 || entropy.entropy_bits() < MIN_ENTROPY_BITS {
            return Err(AuthError::WeakPassphrase(
                "New passphrase is too weak. Please use a longer, more complex passphrase.".to_string(),
            ));
        }

        // Hash new passphrase
        let salt = SaltString::generate(&mut OsRng);
        let passphrase_hash = self
            .argon2
            .hash_password(new_passphrase.as_bytes(), &salt)
            .map_err(|_| AuthError::InternalError)?
            .to_string();

        // Update user
        let mut user = user;
        user.passphrase_hash = passphrase_hash;
        user.password_reset_token = None;
        user.password_reset_expires = None;
        user.updated_at = OffsetDateTime::now_utc();
        
        self.repository.update_user(&user).await
    }
}