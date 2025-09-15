use tonic::{Request, Response, Status};
use tracing::{info, error};

use crate::proxy::ServiceProxies;
use super::{GatewayServer, map_error, get_auth_token};

// Generated protobuf code
tonic::include_proto!("selfie.post.v1");

#[tonic::async_trait]
impl post_service_server::PostService for GatewayServer {
    async fn create_post(
        &self,
        request: Request<CreatePostRequest>
    ) -> Result<Response<CreatePostResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.post.write().await;
        client
            .create_post(request)
            .await
            .map_err(map_error)
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>
    ) -> Result<Response<GetPostResponse>, Status> {
        let mut client = self.proxies.post.write().await;
        client
            .get_post(request)
            .await
            .map_err(map_error)
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>
    ) -> Result<Response<UpdatePostResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.post.write().await;
        client
            .update_post(request)
            .await
            .map_err(map_error)
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>
    ) -> Result<Response<DeletePostResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.post.write().await;
        client
            .delete_post(request)
            .await
            .map_err(map_error)
    }

    async fn list_user_posts(
        &self,
        request: Request<ListUserPostsRequest>
    ) -> Result<Response<ListUserPostsResponse>, Status> {
        let mut client = self.proxies.post.write().await;
        client
            .list_user_posts(request)
            .await
            .map_err(map_error)
    }

    async fn get_feed(
        &self,
        request: Request<GetFeedRequest>
    ) -> Result<Response<GetFeedResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.post.write().await;
        client
            .get_feed(request)
            .await
            .map_err(map_error)
    }

    async fn like_post(
        &self,
        request: Request<LikePostRequest>
    ) -> Result<Response<LikePostResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.post.write().await;
        client
            .like_post(request)
            .await
            .map_err(map_error)
    }

    async fn unlike_post(
        &self,
        request: Request<UnlikePostRequest>
    ) -> Result<Response<UnlikePostResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.post.write().await;
        client
            .unlike_post(request)
            .await
            .map_err(map_error)
    }

    async fn get_likes(
        &self,
        request: Request<GetLikesRequest>
    ) -> Result<Response<GetLikesResponse>, Status> {
        let mut client = self.proxies.post.write().await;
        client
            .get_likes(request)
            .await
            .map_err(map_error)
    }

    async fn add_comment(
        &self,
        request: Request<AddCommentRequest>
    ) -> Result<Response<AddCommentResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.post.write().await;
        client
            .add_comment(request)
            .await
            .map_err(map_error)
    }

    async fn delete_comment(
        &self,
        request: Request<DeleteCommentRequest>
    ) -> Result<Response<DeleteCommentResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.post.write().await;
        client
            .delete_comment(request)
            .await
            .map_err(map_error)
    }

    async fn get_comments(
        &self,
        request: Request<GetCommentsRequest>
    ) -> Result<Response<GetCommentsResponse>, Status> {
        let mut client = self.proxies.post.write().await;
        client
            .get_comments(request)
            .await
            .map_err(map_error)
    }
}