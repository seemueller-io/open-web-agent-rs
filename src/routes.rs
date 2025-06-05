use crate::handlers::{not_found::handle_not_found, ui::serve_ui};
use axum::routing::{get, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use crate::counter::Counter;

pub fn create_router() -> Router {

    let service = StreamableHttpService::new(
        Counter::new,
        LocalSessionManager::default().into(),
        Default::default(),
    );
    
    
    Router::new()
        .nest_service("/mcp", service)
        .route("/", get(serve_ui))
        .route("/health", get(health))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(tower_http::cors::CorsLayer::very_permissive())
        .fallback(handle_not_found)
}

async fn health() -> String {
    return "ok".to_string();
}
