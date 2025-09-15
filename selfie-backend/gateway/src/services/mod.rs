use tonic::{Request, Response, Status};
use tracing::{error, info};

pub mod auth;
pub mod user;
pub mod post;
pub mod media;
pub mod chat;

#[derive(Clone)]
pub struct GatewayServer {
    proxies: crate::proxy::ServiceProxies,
}

impl GatewayServer {
    pub fn new(proxies: crate::proxy::ServiceProxies) -> Self {
        Self { proxies }
    }
}

// Helper function to map errors to gRPC status
pub(crate) fn map_error(err: impl std::error::Error) -> Status {
    error!("Service error: {}", err);
    match err.to_string() {
        e if e.contains("not found") => Status::not_found(e),
        e if e.contains("invalid") => Status::invalid_argument(e),
        e if e.contains("unauthenticated") => Status::unauthenticated(e),
        e if e.contains("permission denied") => Status::permission_denied(e),
        e => Status::internal(format!("Internal error: {}", e)),
    }
}

// Helper function to extract metadata from request
pub(crate) fn get_auth_token(req: &Request<()>) -> Result<String, Status> {
    let metadata = req.metadata();
    let auth_header = metadata
        .get("authorization")
        .ok_or_else(|| Status::unauthenticated("Missing authorization header"))?;
    
    let auth_str = auth_header
        .to_str()
        .map_err(|_| Status::unauthenticated("Invalid authorization header"))?;

    if !auth_str.starts_with("Bearer ") {
        return Err(Status::unauthenticated("Invalid authorization format"));
    }

    Ok(auth_str[7..].to_string())
}

// Helper trait for proxy response mapping
#[async_trait::async_trait]
pub(crate) trait ProxyService {
    type Request;
    type Response;
    type Error: std::error::Error;

    async fn proxy_call(&self, req: Request<Self::Request>) -> Result<Response<Self::Response>, Status>;
}