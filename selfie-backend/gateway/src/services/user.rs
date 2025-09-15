use tonic::{Request, Response, Status};
use tracing::{info, error};

use crate::proxy::ServiceProxies;
use super::{GatewayServer, map_error, get_auth_token};

// Generated protobuf code
tonic::include_proto!("selfie.user.v1");

#[tonic::async_trait]
impl user_service_server::UserService for GatewayServer {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>
    ) -> Result<Response<GetUserResponse>, Status> {
        let mut client = self.proxies.user.write().await;
        client
            .get_user(request)
            .await
            .map_err(map_error)
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>
    ) -> Result<Response<UpdateUserResponse>, Status> {
        // Verify auth token
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.user.write().await;
        client
            .update_user(request)
            .await
            .map_err(map_error)
    }

    async fn update_avatar(
        &self,
        request: Request<UpdateAvatarRequest>
    ) -> Result<Response<UpdateAvatarResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.user.write().await;
        client
            .update_avatar(request)
            .await
            .map_err(map_error)
    }

    async fn get_profile(
        &self,
        request: Request<GetProfileRequest>
    ) -> Result<Response<GetProfileResponse>, Status> {
        let mut client = self.proxies.user.write().await;
        client
            .get_profile(request)
            .await
            .map_err(map_error)
    }

    async fn update_profile(
        &self,
        request: Request<UpdateProfileRequest>
    ) -> Result<Response<UpdateProfileResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.user.write().await;
        client
            .update_profile(request)
            .await
            .map_err(map_error)
    }

    async fn search_users(
        &self,
        request: Request<SearchUsersRequest>
    ) -> Result<Response<SearchUsersResponse>, Status> {
        let mut client = self.proxies.user.write().await;
        client
            .search_users(request)
            .await
            .map_err(map_error)
    }

    async fn block_user(
        &self,
        request: Request<BlockUserRequest>
    ) -> Result<Response<BlockUserResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.user.write().await;
        client
            .block_user(request)
            .await
            .map_err(map_error)
    }

    async fn unblock_user(
        &self,
        request: Request<UnblockUserRequest>
    ) -> Result<Response<UnblockUserResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.user.write().await;
        client
            .unblock_user(request)
            .await
            .map_err(map_error)
    }

    async fn get_blocked_users(
        &self,
        request: Request<GetBlockedUsersRequest>
    ) -> Result<Response<GetBlockedUsersResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.user.write().await;
        client
            .get_blocked_users(request)
            .await
            .map_err(map_error)
    }
}