use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_http::compression::CompressionLayer;

mod handlers;
mod models;
mod routes;
mod storage;

#[tokio::main]
async fn main() {
    // Initialize the application router
    let app = routes::create_router()
        // Serve static files from the static/ directory
        .nest_service("/static", ServeDir::new("static"))
        // Add gzip compression for responses
        .layer(CompressionLayer::new());

    // Bind to localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 PPST Academy server listening on http://{}", addr);

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
