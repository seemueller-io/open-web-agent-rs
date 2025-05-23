use crate::handlers::webhooks::handle_webhooks_post;
use crate::handlers::{
    error::handle_not_found,
    ui::serve_ui
    ,
    webhooks::handle_webhooks,
};
use crate::session_identify::session_identify;
use axum::extract::Request;
use axum::response::Response;
use axum::routing::post;
// src/routes.rs
use axum::routing::{get, Router};
use http::header::AUTHORIZATION;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::fmt;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentUser {
    pub(crate) sub: String,
    pub name: String,
    pub email: String,
    pub exp: Number,
    pub id: String,
    pub aud: String,
}

impl fmt::Display for CurrentUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CurrentUser {{ id: {}, name: {}, email: {}, sub: {}, aud: {}, exp: {} }}",
            self.id, self.name, self.email, self.sub, self.aud, self.exp
        )
    }
}

pub fn create_router() -> Router {

    Router::new()
        .route("/", get(serve_ui))
        // request a stream resource
        .route("/api/webhooks", post(handle_webhooks_post))
        // consume a stream resource
        .route("/webhooks/:stream_id", get(handle_webhooks))
        .route_layer(axum::middleware::from_fn(auth))
        .route("/health", get(health))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        // left for smoke testing
        // .route("/api/status", get(handle_status))
        .fallback(handle_not_found)
}

async fn health() -> String {
    return "ok".to_string();
}

async fn auth(mut req: Request, next: axum::middleware::Next) -> Result<Response, StatusCode> {
    let session_token_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header_value| header_value.to_str().ok());

    let session_token_parts= session_token_header.expect("No credentials").split(" ").collect::<Vec<&str>>();

    let session_token = session_token_parts.get(1);


    // log::info!("session_token: {:?}", session_token);

    let session_token = session_token.expect("Unauthorized: No credentials supplied");

    let result =
        if let Some(current_user) = authorize_current_user(&*session_token).await {
            // info!("current user: {}", current_user);
            // insert the current user into a request extension so the handler can
            // extract it
            req.extensions_mut().insert(current_user);
            Ok(next.run(req).await)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        };
    result
}


async fn authorize_current_user(
    session_token: &str,
) -> Option<CurrentUser> {
    let session_identity = session_identify(session_token)
        .await
        .unwrap();

    // println!("current_user: {:?}", session_identity.user);

    Some(serde_json::from_value::<CurrentUser>(session_identity.user).unwrap())
}