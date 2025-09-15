use std::sync::Arc;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{error::AuthError, service::jwt::JwtService};

#[derive(Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
}

pub async fn auth_middleware<B>(
    State(jwt_service): State<Arc<JwtService>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AuthError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AuthError::AuthenticationError)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::AuthenticationError);
    }

    let token = &auth_header[7..];
    let user_id = jwt_service.verify_token(token, "access")?;

    request.extensions_mut().insert(AuthContext { user_id });

    Ok(next.run(request).await)
}

pub async fn refresh_token_middleware<B>(
    State(jwt_service): State<Arc<JwtService>>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, AuthError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AuthError::AuthenticationError)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::AuthenticationError);
    }

    let token = &auth_header[7..];
    jwt_service.verify_token(token, "refresh")?;

    Ok(next.run(request).await)
}