use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
}

impl From<GatewayError> for Status {
    fn from(err: GatewayError) -> Self {
        match err {
            GatewayError::Internal(msg) => Status::internal(msg),
            GatewayError::ServiceUnavailable(msg) => Status::unavailable(msg),
            GatewayError::InvalidRequest(msg) => Status::invalid_argument(msg),
            GatewayError::AuthenticationFailed(msg) => Status::unauthenticated(msg),
        }
    }
}