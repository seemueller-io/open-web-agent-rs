use crate::handlers::webhooks::handle_webhooks_post;
use crate::handlers::{error::handle_not_found, ui::serve_ui, webhooks::handle_webhooks};
use axum::routing::post;
use axum::routing::{get, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(serve_ui))
        // create an agent
        .route("/api/agents", post(handle_webhooks_post))
        // connect the agent
        .route("/agents/:agent_id", get(handle_webhooks))
        .route("/health", get(health))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .fallback(handle_not_found)
}

async fn health() -> String {
    return "ok".to_string();
}
