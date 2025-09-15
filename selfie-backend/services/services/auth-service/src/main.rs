mod auth;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;

use axum::{
    error_handling::HandleErrorLayer,
    http::{HeaderValue, Method, StatusCode},
    Extension, Router,
};
use foundationdb::Database;
use std::{net::SocketAddr, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    request_id::RequestIdLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::{info, Level};

use crate::{db::FdbPool, handlers::auth_routes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Initializing auth service...");

    // Initialize FoundationDB
    foundationdb::init().expect("Failed to initialize FoundationDB");
    let db = Database::new(None)?;
    let db_pool = FdbPool::new(db);

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin("https://selfie.app".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    // Build middleware stack
    let middleware = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_| async {
            StatusCode::INTERNAL_SERVER_ERROR
        }))
        .layer(TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)))
        .layer(RequestIdLayer::new())
        .layer(CompressionLayer::new())
        .layer(cors)
        .timeout(Duration::from_secs(30))
        .into_inner();

    // Initialize JWT service
    let jwt_service = Arc::new(JwtService::new()?);

    // Initialize email service
    let email_service = Arc::new(EmailService::new(
        std::env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.sendgrid.net".to_string()),
        std::env::var("SMTP_PORT").unwrap_or_else(|_| "587".to_string()).parse().unwrap_or(587),
        std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME"),
        std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD"),
        std::env::var("EMAIL_FROM").unwrap_or_else(|_| "noreply@selfie.app".to_string()),
        std::env::var("APP_URL").unwrap_or_else(|_| "https://selfie.app".to_string()),
    ).await?;

    // Build our application with routes
    let app = Router::new()
        .merge(auth_routes(jwt_service))
        .layer(middleware);

    // Run our service
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Auth service listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
    println!("Server running on {{}}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}}

async fn root() -> &'static str {{
    "Hello, Selfie Backend!"
}}
