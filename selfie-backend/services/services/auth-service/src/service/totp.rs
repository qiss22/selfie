use base32::{decode, encode};
use rand::RngCore;
use totp_rs::{Algorithm, Secret, TOTP, URL};

use crate::error::AuthError;

const TOTP_ISSUER: &str = "Selfie";
const TOTP_DIGITS: usize = 6;
const TOTP_STEP: u64 = 30;
const TOTP_WINDOW: i64 = 1; // Allow 1 step before and after for time skew

pub struct TotpService;

impl TotpService {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_secret(&self) -> Result<String, AuthError> {
        let mut secret = vec![0u8; 32];
        rand::thread_rng()
            .try_fill_bytes(&mut secret)
            .map_err(|_| AuthError::InternalError)?;
        
        Ok(encode(base32::Alphabet::RFC4648 { padding: true }, &secret))
    }

    pub fn generate_provisioning_uri(&self, secret: &str, account_name: &str) -> Result<String, AuthError> {
        let totp = self.create_totp(secret)?;
        
        Ok(totp.get_url(URL {
            issuer: TOTP_ISSUER.to_string(),
            account_name: account_name.to_string(),
            scheme: "otpauth".to_string(),
        }))
    }

    pub fn verify_code(&self, secret: &str, code: &str) -> Result<bool, AuthError> {
        let totp = self.create_totp(secret)?;
        
        // Check if the code is valid within the time window
        Ok(totp.check_current(code, TOTP_WINDOW))
    }

    fn create_totp(&self, secret: &str) -> Result<TOTP, AuthError> {
        let secret_bytes = decode(base32::Alphabet::RFC4648 { padding: true }, secret)
            .map_err(|_| AuthError::InternalError)?;

        TOTP::new(
            Algorithm::SHA1,
            TOTP_DIGITS,
            TOTP_STEP,
            secret_bytes,
            Some(TOTP_ISSUER.to_string()),
            "".to_string(),
        )
        .map_err(|_| AuthError::InternalError)
    }
}