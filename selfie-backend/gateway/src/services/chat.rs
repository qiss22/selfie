use tonic::{Request, Response, Status};
use tracing::{info, error};
use futures::Stream;
use std::pin::Pin;

use crate::proxy::ServiceProxies;
use super::{GatewayServer, map_error, get_auth_token};

// Generated protobuf code
tonic::include_proto!("selfie.chat.v1");

#[tonic::async_trait]
impl chat_service_server::ChatService for GatewayServer {
    async fn create_chat(
        &self,
        request: Request<CreateChatRequest>
    ) -> Result<Response<CreateChatResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.chat.write().await;
        client
            .create_chat(request)
            .await
            .map_err(map_error)
    }

    async fn get_chat(
        &self,
        request: Request<GetChatRequest>
    ) -> Result<Response<GetChatResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.chat.write().await;
        client
            .get_chat(request)
            .await
            .map_err(map_error)
    }

    async fn list_chats(
        &self,
        request: Request<ListChatsRequest>
    ) -> Result<Response<ListChatsResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.chat.write().await;
        client
            .list_chats(request)
            .await
            .map_err(map_error)
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>
    ) -> Result<Response<SendMessageResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.chat.write().await;
        client
            .send_message(request)
            .await
            .map_err(map_error)
    }

    async fn get_messages(
        &self,
        request: Request<GetMessagesRequest>
    ) -> Result<Response<GetMessagesResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.chat.write().await;
        client
            .get_messages(request)
            .await
            .map_err(map_error)
    }

    async fn mark_as_read(
        &self,
        request: Request<MarkAsReadRequest>
    ) -> Result<Response<MarkAsReadResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.chat.write().await;
        client
            .mark_as_read(request)
            .await
            .map_err(map_error)
    }

    async fn delete_message(
        &self,
        request: Request<DeleteMessageRequest>
    ) -> Result<Response<DeleteMessageResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.chat.write().await;
        client
            .delete_message(request)
            .await
            .map_err(map_error)
    }

    type StreamMessagesStream = Pin<Box<dyn Stream<Item = Result<StreamMessagesResponse, Status>> + Send + 'static>>;

    async fn stream_messages(
        &self,
        request: Request<StreamMessagesRequest>
    ) -> Result<Response<Self::StreamMessagesStream>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.chat.write().await;
        client
            .stream_messages(request)
            .await
            .map_err(map_error)
    }
}