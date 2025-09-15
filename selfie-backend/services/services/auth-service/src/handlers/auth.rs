use std::sync::Arc;
use axum::{
    routing::{get, post},
    extract::Query,
    Extension, Json, Router,
};
use validator::Validate;

use crate::{
    api::models::{AuthResponse, LoginRequest, RegisterRequest},
    error::AuthError,
    middleware::{auth::auth_middleware, auth::refresh_token_middleware},
    repository::fdb::FdbUserRepository,
    service::{auth::AuthService, jwt::JwtService, models::User},
};

pub fn auth_routes(jwt_service: Arc<JwtService>) -> Router {
    let repository = Arc::new(FdbUserRepository::new(
        foundationdb::Database::new(None).expect("Failed to create FDB database"),
    ));
    let auth_service = Arc::new(AuthService::new(repository, jwt_service.clone()));

    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/verify-email", get(verify_email))
        .route("/request-password-reset", post(request_password_reset))
        .route("/reset-password", post(reset_password))
        .route(
            "/me",
            get(get_current_user).route_layer(axum::middleware::from_fn_with_state(
                jwt_service.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/2fa/setup",
            post(setup_totp).route_layer(axum::middleware::from_fn_with_state(
                jwt_service.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/2fa/enable",
            post(enable_totp).route_layer(axum::middleware::from_fn_with_state(
                jwt_service.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/2fa/disable",
            post(disable_totp).route_layer(axum::middleware::from_fn_with_state(
                jwt_service,
                auth_middleware,
            )),
        )
        .layer(Extension(auth_service))
}

async fn register(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<User>, AuthError> {
    req.validate()
        .map_err(|_| AuthError::InvalidCredentials)?;

    let user = auth_service.register(req).await?;
    Ok(Json(user))
}

async fn login(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    req.validate()
        .map_err(|_| AuthError::InvalidCredentials)?;

    let token = auth_service.login(req).await?;
    Ok(Json(token))
}

async fn refresh_token(
    Extension(auth_service): Extension<Arc<AuthService>>,
    headers: axum::http::header::HeaderMap,
) -> Result<Json<AuthResponse>, AuthError> {
    let refresh_token = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .ok_or(AuthError::AuthenticationError)?;

    let new_tokens = auth_service.refresh_token(refresh_token).await?;
    Ok(Json(new_tokens))
}

async fn get_current_user(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Extension(auth_context): Extension<crate::middleware::auth::AuthContext>,
) -> Result<Json<User>, AuthError> {
    let user = auth_service
        .repository
        .get_user_by_id(&auth_context.user_id)
        .await?
        .ok_or(AuthError::UserNotFound)?;

    Ok(Json(user))
}

async fn setup_totp(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Extension(auth_context): Extension<crate::middleware::auth::AuthContext>,
) -> Result<Json<TotpSecretResponse>, AuthError> {
    let response = auth_service.setup_totp(auth_context.user_id).await?;
    Ok(Json(response))
}

async fn enable_totp(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Extension(auth_context): Extension<crate::middleware::auth::AuthContext>,
    Json(req): Json<EnableTotpRequest>,
) -> Result<Json<()>, AuthError> {
    req.validate().map_err(|_| AuthError::InvalidCredentials)?;
    auth_service.enable_totp(auth_context.user_id, req).await?;
    Ok(Json(()))
}

async fn disable_totp(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Extension(auth_context): Extension<crate::middleware::auth::AuthContext>,
    Json(code): Json<String>,
) -> Result<Json<()>, AuthError> {
    auth_service.disable_totp(auth_context.user_id, &code).await?;
    Ok(Json(()))
}

async fn verify_email(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Query(token): Query<String>,
) -> Result<Json<()>, AuthError> {
    auth_service.verify_email(&token).await?;
    Ok(Json(()))
}

async fn request_password_reset(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Json(email): Json<String>,
) -> Result<Json<()>, AuthError> {
    auth_service.initiate_password_reset(&email).await?;
    Ok(Json(()))
}

async fn reset_password(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Json(req): Json<PasswordResetRequest>,
) -> Result<Json<()>, AuthError> {
    auth_service.reset_password(&req.token, &req.new_passphrase).await?;
    Ok(Json(()))
}