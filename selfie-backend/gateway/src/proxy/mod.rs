use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::Channel;

pub mod auth;
pub mod user;
pub mod post;
pub mod media;
pub mod chat;

#[derive(Clone)]
pub struct ServiceProxies {
    pub auth: Arc<RwLock<auth::AuthServiceClient>>,
    pub user: Arc<RwLock<user::UserServiceClient>>,
    pub post: Arc<RwLock<post::PostServiceClient>>,
    pub media: Arc<RwLock<media::MediaServiceClient>>,
    pub chat: Arc<RwLock<chat::ChatServiceClient>>,
}

impl ServiceProxies {
    pub async fn new(config: &crate::config::ServiceDiscoveryConfig) -> Result<Self, tonic::transport::Error> {
        Ok(Self {
            auth: Arc::new(RwLock::new(
                auth::AuthServiceClient::connect(config.auth_service.clone()).await?,
            )),
            user: Arc::new(RwLock::new(
                user::UserServiceClient::connect(config.user_service.clone()).await?,
            )),
            post: Arc::new(RwLock::new(
                post::PostServiceClient::connect(config.post_service.clone()).await?,
            )),
            media: Arc::new(RwLock::new(
                media::MediaServiceClient::connect(config.media_service.clone()).await?,
            )),
            chat: Arc::new(RwLock::new(
                chat::ChatServiceClient::connect(config.chat_service.clone()).await?,
            )),
        })
    }
}

// Service client type definitions
pub type AuthServiceClient = auth::AuthServiceClient<Channel>;
pub type UserServiceClient = user::UserServiceClient<Channel>;
pub type PostServiceClient = post::PostServiceClient<Channel>;
pub type MediaServiceClient = media::MediaServiceClient<Channel>;
pub type ChatServiceClient = chat::ChatServiceClient<Channel>;