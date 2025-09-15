use std::sync::Arc;
use ed25519_dalek::{Keypair, SecretKey, Signer};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::error::AuthError;

const ACCESS_TOKEN_DURATION: i64 = 900; // 15 minutes in seconds
const REFRESH_TOKEN_DURATION: i64 = 2592000; // 30 days in seconds

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,         // User ID
    exp: i64,           // Expiration time
    iat: i64,           // Issued at
    jti: String,        // JWT ID
    #[serde(rename = "type")]
    token_type: String, // Token type (access or refresh)
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    signing_keypair: Keypair,
}

impl JwtService {
    pub fn new() -> Result<Self, AuthError> {
        // In production, load these from secure storage/HSM
        let secret = ed25519_dalek::SigningKey::generate(&mut rand::thread_rng());
        let public = secret.verifying_key();
        let keypair = Keypair {
            secret,
            public,
        };

        Ok(Self {
            encoding_key: EncodingKey::from_ed_der(&keypair.secret.to_bytes()),
            decoding_key: DecodingKey::from_ed_der(&keypair.public.to_bytes()),
            signing_keypair: keypair,
        })
    }

    pub fn generate_access_token(&self, user_id: Uuid) -> Result<String, AuthError> {
        let now = OffsetDateTime::now_utc();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (now + Duration::seconds(ACCESS_TOKEN_DURATION)).unix_timestamp(),
            iat: now.unix_timestamp(),
            jti: Uuid::new_v4().to_string(),
            token_type: "access".to_string(),
        };

        encode(
            &Header::new(jsonwebtoken::Algorithm::EdDSA),
            &claims,
            &self.encoding_key,
        )
        .map_err(|_| AuthError::InternalError)
    }

    pub fn generate_refresh_token(&self, user_id: Uuid) -> Result<String, AuthError> {
        let now = OffsetDateTime::now_utc();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (now + Duration::seconds(REFRESH_TOKEN_DURATION)).unix_timestamp(),
            iat: now.unix_timestamp(),
            jti: Uuid::new_v4().to_string(),
            token_type: "refresh".to_string(),
        };

        encode(
            &Header::new(jsonwebtoken::Algorithm::EdDSA),
            &claims,
            &self.encoding_key,
        )
        .map_err(|_| AuthError::InternalError)
    }

    pub fn verify_token(&self, token: &str, expected_type: &str) -> Result<Uuid, AuthError> {
        let mut validation = Validation::new(jsonwebtoken::Algorithm::EdDSA);
        validation.set_required_spec_claims(&["sub", "exp", "iat", "jti", "type"]);

        let token_data: TokenData<Claims> = decode(
            token,
            &self.decoding_key,
            &validation,
        ).map_err(|_| AuthError::InvalidToken)?;

        if token_data.claims.token_type != expected_type {
            return Err(AuthError::InvalidToken);
        }

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(user_id)
    }

    // Sign arbitrary data using the service's keypair
    pub fn sign_data(&self, data: &[u8]) -> Vec<u8> {
        self.signing_keypair.sign(data).to_bytes().to_vec()
    }

    // Verify signed data
    pub fn verify_signature(&self, data: &[u8], signature: &[u8]) -> bool {
        if let Ok(sig) = ed25519_dalek::Signature::from_bytes(signature) {
            self.signing_keypair.public.verify_strict(data, &sig).is_ok()
        } else {
            false
        }
    }
}