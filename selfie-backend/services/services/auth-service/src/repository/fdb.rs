use std::sync::Arc;
use async_trait::async_trait;
use foundationdb::{Database, RangeOption};
use uuid::Uuid;

use crate::{
    error::AuthError,
    repository::UserRepository,
    service::models::User,
};

const USER_PREFIX: &[u8] = b"user";
const EMAIL_INDEX_PREFIX: &[u8] = b"email_idx";
const VERIFICATION_TOKEN_PREFIX: &[u8] = b"verify";
const RESET_TOKEN_PREFIX: &[u8] = b"reset";

pub struct FdbUserRepository {
    db: Arc<Database>,
}

impl FdbUserRepository {
    pub fn new(db: Database) -> Self {
        Self {
            db: Arc::new(db),
        }
    }

    fn make_user_key(id: &Uuid) -> Vec<u8> {
        let mut key = Vec::with_capacity(USER_PREFIX.len() + 16);
        key.extend_from_slice(USER_PREFIX);
        key.extend_from_slice(id.as_bytes());
        key
    }

    fn make_email_key(email: &str) -> Vec<u8> {
        let mut key = Vec::with_capacity(EMAIL_INDEX_PREFIX.len() + email.len());
        key.extend_from_slice(EMAIL_INDEX_PREFIX);
        key.extend_from_slice(email.as_bytes());
        key
    }

    fn make_verification_key(token: &str) -> Vec<u8> {
        let mut key = Vec::with_capacity(VERIFICATION_TOKEN_PREFIX.len() + token.len());
        key.extend_from_slice(VERIFICATION_TOKEN_PREFIX);
        key.extend_from_slice(token.as_bytes());
        key
    }

    fn make_reset_key(token: &str) -> Vec<u8> {
        let mut key = Vec::with_capacity(RESET_TOKEN_PREFIX.len() + token.len());
        key.extend_from_slice(RESET_TOKEN_PREFIX);
        key.extend_from_slice(token.as_bytes());
        key
    }
}

#[async_trait]
impl UserRepository for FdbUserRepository {
    async fn create_user(&self, user: &User) -> Result<(), AuthError> {
        let db = self.db.clone();
        
        db.run(|tr| async move {
            // Check if email already exists
            let email_key = Self::make_email_key(&user.email);
            if tr.get(&email_key).await?.is_some() {
                return Err(AuthError::UserExists);
            }

            // Store user data
            let user_key = Self::make_user_key(&user.id);
            let user_bytes = serde_json::to_vec(&user)
                .map_err(|_| AuthError::InternalError)?;
            
            tr.set(&user_key, &user_bytes);
            tr.set(&email_key, &user.id.as_bytes());

            // Set verification token index if exists
            if let Some(token) = &user.email_verification_token {
                let verify_key = Self::make_verification_key(token);
                tr.set(&verify_key, &user.id.as_bytes());
            }

            Ok(())
        }).await
    }

    async fn get_user_by_id(&self, id: &Uuid) -> Result<Option<User>, AuthError> {
        let db = self.db.clone();
        
        db.run(|tr| async move {
            let key = Self::make_user_key(id);
            let bytes = tr.get(&key).await?;
            
            match bytes {
                Some(bytes) => {
                    let user = serde_json::from_slice(&bytes)
                        .map_err(|_| AuthError::InternalError)?;
                    Ok(Some(user))
                }
                None => Ok(None),
            }
        }).await
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AuthError> {
        let db = self.db.clone();
        
        db.run(|tr| async move {
            let email_key = Self::make_email_key(email);
            
            if let Some(user_id_bytes) = tr.get(&email_key).await? {
                let user_id = Uuid::from_slice(&user_id_bytes)
                    .map_err(|_| AuthError::InternalError)?;
                    
                let user_key = Self::make_user_key(&user_id);
                let user_bytes = tr.get(&user_key).await?
                    .ok_or(AuthError::InternalError)?;
                    
                let user = serde_json::from_slice(&user_bytes)
                    .map_err(|_| AuthError::InternalError)?;
                    
                Ok(Some(user))
            } else {
                Ok(None)
            }
        }).await
    }

    async fn update_user(&self, user: &User) -> Result<(), AuthError> {
        let db = self.db.clone();
        
        db.run(|tr| async move {
            let user_key = Self::make_user_key(&user.id);
            let user_bytes = serde_json::to_vec(&user)
                .map_err(|_| AuthError::InternalError)?;
                
            tr.set(&user_key, &user_bytes);

            // Update indices
            if let Some(token) = &user.email_verification_token {
                let verify_key = Self::make_verification_key(token);
                tr.set(&verify_key, &user.id.as_bytes());
            }

            if let Some(token) = &user.password_reset_token {
                let reset_key = Self::make_reset_key(token);
                tr.set(&reset_key, &user.id.as_bytes());
            }

            Ok(())
        }).await
    }

    async fn get_user_by_verification_token(&self, token: &str) -> Result<Option<User>, AuthError> {
        let db = self.db.clone();
        
        db.run(|tr| async move {
            let verify_key = Self::make_verification_key(token);
            
            if let Some(user_id_bytes) = tr.get(&verify_key).await? {
                let user_id = Uuid::from_slice(&user_id_bytes)
                    .map_err(|_| AuthError::InternalError)?;
                    
                let user_key = Self::make_user_key(&user_id);
                let user_bytes = tr.get(&user_key).await?
                    .ok_or(AuthError::InternalError)?;
                    
                let user = serde_json::from_slice(&user_bytes)
                    .map_err(|_| AuthError::InternalError)?;
                    
                Ok(Some(user))
            } else {
                Ok(None)
            }
        }).await
    }

    async fn get_user_by_reset_token(&self, token: &str) -> Result<Option<User>, AuthError> {
        let db = self.db.clone();
        
        db.run(|tr| async move {
            let reset_key = Self::make_reset_key(token);
            
            if let Some(user_id_bytes) = tr.get(&reset_key).await? {
                let user_id = Uuid::from_slice(&user_id_bytes)
                    .map_err(|_| AuthError::InternalError)?;
                    
                let user_key = Self::make_user_key(&user_id);
                let user_bytes = tr.get(&user_key).await?
                    .ok_or(AuthError::InternalError)?;
                    
                let user = serde_json::from_slice(&user_bytes)
                    .map_err(|_| AuthError::InternalError)?;
                    
                Ok(Some(user))
            } else {
                Ok(None)
            }
        }).await
    }
}