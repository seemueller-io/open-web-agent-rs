use crate::config::{Runtime};
use crate::routes::create_router;
use crate::setup::init_logging;

mod config;
mod routes;
mod setup;
mod handlers;
mod agents;
mod utils;

#[tokio::main]
async fn main() {
    init_logging();

    Runtime::configure();

    let router = create_router();

    let addr = "0.0.0.0:3006";
    tracing::info!("Attempting to bind server to {}", addr);

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
    axum::serve(listener, router.into_make_service()).await.unwrap();
}
