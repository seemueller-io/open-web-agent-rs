use crate::handlers::agents::create_agent;
use crate::handlers::{not_found::handle_not_found, ui::serve_ui, agents::use_agent};
use axum::routing::post;
use axum::routing::{get, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(serve_ui))
        // create an agent
        .route("/api/agents", post(create_agent))
        // connect the agent
        .route("/agents/:agent_id", get(use_agent))
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
