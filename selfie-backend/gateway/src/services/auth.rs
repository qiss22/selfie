use tonic::{Request, Response, Status};
use tracing::{info, error};

use crate::proxy::ServiceProxies;
use super::{GatewayServer, map_error};

// Generated protobuf code will be included here by build.rs
tonic::include_proto!("selfie.auth.v1");

#[tonic::async_trait]
impl auth_service_server::AuthService for GatewayServer {
    async fn register(
        &self,
        request: Request<RegisterRequest>
    ) -> Result<Response<RegisterResponse>, Status> {
        let mut client = self.proxies.auth.write().await;
        client
            .register(request)
            .await
            .map_err(map_error)
    }

    async fn login(
        &self,
        request: Request<LoginRequest>
    ) -> Result<Response<LoginResponse>, Status> {
        let mut client = self.proxies.auth.write().await;
        client
            .login(request)
            .await
            .map_err(map_error)
    }

    async fn refresh(
        &self,
        request: Request<RefreshRequest>
    ) -> Result<Response<RefreshResponse>, Status> {
        let mut client = self.proxies.auth.write().await;
        client
            .refresh(request)
            .await
            .map_err(map_error)
    }

    async fn verify2_fa(
        &self,
        request: Request<Verify2FaRequest>
    ) -> Result<Response<Verify2FaResponse>, Status> {
        let mut client = self.proxies.auth.write().await;
        client
            .verify2_fa(request)
            .await
            .map_err(map_error)
    }

    async fn setup2_fa(
        &self,
        request: Request<Setup2FaRequest>
    ) -> Result<Response<Setup2FaResponse>, Status> {
        let mut client = self.proxies.auth.write().await;
        client
            .setup2_fa(request)
            .await
            .map_err(map_error)
    }

    async fn validate_token(
        &self,
        request: Request<ValidateTokenRequest>
    ) -> Result<Response<ValidateTokenResponse>, Status> {
        let mut client = self.proxies.auth.write().await;
        client
            .validate_token(request)
            .await
            .map_err(map_error)
    }

    async fn reset_password(
        &self,
        request: Request<ResetPasswordRequest>
    ) -> Result<Response<ResetPasswordResponse>, Status> {
        let mut client = self.proxies.auth.write().await;
        client
            .reset_password(request)
            .await
            .map_err(map_error)
    }

    async fn verify_email(
        &self,
        request: Request<VerifyEmailRequest>
    ) -> Result<Response<VerifyEmailResponse>, Status> {
        let mut client = self.proxies.auth.write().await;
        client
            .verify_email(request)
            .await
            .map_err(map_error)
    }
}