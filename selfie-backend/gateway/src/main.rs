use anyhow::Result;
use tonic::transport::Server;
use tracing::{info, Level};
use tower::ServiceBuilder;

mod config;
mod error;
mod proxy;
mod services;
mod middleware;

use middleware::{AuthMiddlewareLayer, LoggingMiddlewareLayer};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Starting Selfie gRPC Gateway...");

    // Load configuration
    let config = config::load_config()?;
    
    // Initialize metrics
    let metrics_recorder = metrics_exporter_prometheus::PrometheusBuilder::new()
        .with_http_listener(config.metrics_addr.parse()?)
        .build()?;
    
    // Initialize service proxies
    let proxies = proxy::ServiceProxies::new(&config.service_discovery).await?;
    
    // Create gateway server instance
    let gateway = services::GatewayServer::new(proxies);
    
    // Create middleware stack
    let middleware = ServiceBuilder::new()
        .layer(LoggingMiddlewareLayer)
        .layer(AuthMiddlewareLayer)
        .into_inner();

    // Start the gRPC server
    let addr = format!("{}:{}", config.host, config.port).parse()?;
    info!("Gateway listening on {}", addr);

    Server::builder()
        .accept_http1(true) // Enable HTTP/1.1 for health checks
        .layer(middleware)
        // Add service implementations
        .add_service(services::auth::AuthServer::new(gateway.clone()))
        .add_service(services::user::UserServer::new(gateway.clone()))
        .add_service(services::post::PostServer::new(gateway.clone()))
        .add_service(services::media::MediaServer::new(gateway.clone()))
        .add_service(services::chat::ChatServer::new(gateway.clone()))
        // Add health service
        .add_service(tonic_health::server::HealthServer::new(
            tonic_health::server::HealthReporter::new(),
        ))
        .serve(addr)
        .await?;

    Ok(())
}