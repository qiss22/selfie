use axum::{{routing::get, Router}};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {{
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on {{}}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}}

async fn root() -> &'static str {{
    "Hello, Selfie Backend!"
}}
