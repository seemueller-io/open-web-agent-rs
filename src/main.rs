// src/main.rs
use crate::config::AppConfig;
use crate::routes::create_router;
use crate::setup::init_logging;

mod config;
mod routes;
mod setup;
mod handlers;
mod agents;
mod genaiscript;
mod utils;
mod session_identify;

#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();

    // Load configuration
    let config = AppConfig::new();

    // Create router with all routes
    let app = create_router();

    // Start core
    let addr = "0.0.0.0:3006";
    tracing::info!("Attempting to bind core to {}", addr);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => {
            tracing::info!("Successfully bound to {}", l.local_addr().unwrap());
            l
        }
        Err(e) => {
            tracing::error!("Failed to bind to {}: {}", addr, e);
            panic!("Server failed to start");
        }
    };

    tracing::info!("Server starting on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
