use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;
use tower::{Layer, Service};
use tonic::{Request, Status};
use tracing::{info, warn, error};
use metrics::{counter, histogram};

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> AuthMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddlewareLayer;

impl<S> Layer<S> for AuthMiddlewareLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuthMiddleware::new(service)
    }
}

impl<S, ReqBody> Service<Request<ReqBody>> for AuthMiddleware<S>
where
    S: Service<Request<ReqBody>>,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let start = std::time::Instant::now();
        let path = req.uri().path().to_string();

        // Skip auth for health check and non-authenticated endpoints
        if path.contains("/health") || path.contains("/auth/login") || path.contains("/auth/register") {
            let fut = self.inner.call(req);
            return Box::pin(async move {
                let result = fut.await;
                let duration = start.elapsed();
                histogram!("request_duration_ms", duration.as_millis() as f64, "path" => path);
                result
            });
        }

        // Verify auth token
        let auth_result = req.metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));

        match auth_result {
            Some(token) => {
                // Add token validation here
                info!("Request authenticated with token");
                counter!("requests_authenticated_total", "path" => path.clone());
                
                let fut = self.inner.call(req);
                Box::pin(async move {
                    let result = fut.await;
                    let duration = start.elapsed();
                    histogram!("request_duration_ms", duration.as_millis() as f64, "path" => path);
                    result
                })
            }
            None => {
                warn!("Unauthenticated request to {}", path);
                counter!("requests_unauthenticated_total", "path" => path);
                Box::pin(async move {
                    Err(Status::unauthenticated("Missing or invalid authentication token").into())
                })
            }
        }
    }
}

// Logging middleware
#[derive(Clone)]
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S> LoggingMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

#[derive(Clone)]
pub struct LoggingMiddlewareLayer;

impl<S> Layer<S> for LoggingMiddlewareLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        LoggingMiddleware::new(service)
    }
}

impl<S, ReqBody> Service<Request<ReqBody>> for LoggingMiddleware<S>
where
    S: Service<Request<ReqBody>>,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let start = std::time::Instant::now();
        let path = req.uri().path().to_string();
        let method = req.method().clone();

        info!(
            "Request started: {} {}",
            method,
            path
        );

        let fut = self.inner.call(req);
        Box::pin(async move {
            let result = fut.await;
            let duration = start.elapsed();
            
            match &result {
                Ok(_) => {
                    info!(
                        "Request completed: {} {} ({:?})",
                        method,
                        path,
                        duration
                    );
                }
                Err(e) => {
                    error!(
                        "Request failed: {} {} ({:?}) - Error: {}",
                        method,
                        path,
                        duration,
                        e
                    );
                }
            }
            
            result
        })
    }
}