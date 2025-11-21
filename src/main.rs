use std::env;
use std::net::SocketAddr;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod models;
mod routes;
mod storage;

#[tokio::main]
async fn main() {
    // Initialize tracing/logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ppst_academy=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize the application router
    let app = routes::create_router()
        // Serve static files from the static/ directory
        .nest_service("/static", ServeDir::new("static"))
        // Add gzip compression for responses
        .layer(CompressionLayer::new());

    // Determine port from environment (PORT or PPST_PORT), default to 3000
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .or_else(|| {
            env::var("PPST_PORT")
                .ok()
                .and_then(|v| v.parse::<u16>().ok())
        })
        .unwrap_or(3000);

    // Bind to all network interfaces (0.0.0.0) to allow external access
    // Use 127.0.0.1 for localhost-only access
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 PPST Academy server listening on http://{}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Failed to start server");
}
