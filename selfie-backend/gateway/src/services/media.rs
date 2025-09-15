use tonic::{Request, Response, Status};
use tracing::{info, error};

use crate::proxy::ServiceProxies;
use super::{GatewayServer, map_error, get_auth_token};

// Generated protobuf code
tonic::include_proto!("selfie.media.v1");

#[tonic::async_trait]
impl media_service_server::MediaService for GatewayServer {
    async fn upload_media(
        &self,
        request: Request<UploadMediaRequest>
    ) -> Result<Response<UploadMediaResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.media.write().await;
        client
            .upload_media(request)
            .await
            .map_err(map_error)
    }

    async fn get_media(
        &self,
        request: Request<GetMediaRequest>
    ) -> Result<Response<GetMediaResponse>, Status> {
        let mut client = self.proxies.media.write().await;
        client
            .get_media(request)
            .await
            .map_err(map_error)
    }

    async fn delete_media(
        &self,
        request: Request<DeleteMediaRequest>
    ) -> Result<Response<DeleteMediaResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.media.write().await;
        client
            .delete_media(request)
            .await
            .map_err(map_error)
    }

    async fn generate_thumbnail(
        &self,
        request: Request<GenerateThumbnailRequest>
    ) -> Result<Response<GenerateThumbnailResponse>, Status> {
        let mut client = self.proxies.media.write().await;
        client
            .generate_thumbnail(request)
            .await
            .map_err(map_error)
    }

    async fn optimize_media(
        &self,
        request: Request<OptimizeMediaRequest>
    ) -> Result<Response<OptimizeMediaResponse>, Status> {
        let mut client = self.proxies.media.write().await;
        client
            .optimize_media(request)
            .await
            .map_err(map_error)
    }

    async fn get_upload_url(
        &self,
        request: Request<GetUploadUrlRequest>
    ) -> Result<Response<GetUploadUrlResponse>, Status> {
        let _token = get_auth_token(&request)?;
        
        let mut client = self.proxies.media.write().await;
        client
            .get_upload_url(request)
            .await
            .map_err(map_error)
    }
}