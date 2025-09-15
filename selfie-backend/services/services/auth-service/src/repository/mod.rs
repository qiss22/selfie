use async_trait::async_trait;
use uuid::Uuid;
use crate::service::models::User;
use crate::error::AuthError;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create_user(&self, user: &User) -> Result<(), AuthError>;
    async fn get_user_by_id(&self, id: &Uuid) -> Result<Option<User>, AuthError>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AuthError>;
    async fn update_user(&self, user: &User) -> Result<(), AuthError>;
    async fn get_user_by_verification_token(&self, token: &str) -> Result<Option<User>, AuthError>;
    async fn get_user_by_reset_token(&self, token: &str) -> Result<Option<User>, AuthError>;
}