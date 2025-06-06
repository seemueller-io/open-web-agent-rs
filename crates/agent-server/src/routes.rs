use axum::response::Response;
use crate::handlers::{not_found::handle_not_found};
use axum::routing::{get, Router};
use http::StatusCode;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use rust_embed::Embed;
use crate::agents::Agents;


#[derive(Embed)]
#[folder = "../../node_modules/@modelcontextprotocol/inspector-client/dist"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> axum::response::IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(http::header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

async fn ui_index_handler() -> impl axum::response::IntoResponse {
    StaticFile("index.html")
}

async fn static_handler(uri: http::Uri) -> impl axum::response::IntoResponse {
    let path = uri.path().trim_start_matches("/").to_string();
    StaticFile(path)
}

pub fn create_router() -> Router {

    let mcp_service = StreamableHttpService::new(
        Agents::new,
        LocalSessionManager::default().into(),
        Default::default(),
    );

    Router::new()
        .nest_service("/mcp", mcp_service)
        .route("/health", get(health))
        .route("/", get(ui_index_handler))
        .route("/index.html", get(ui_index_handler))
        .route("/{*path}", get(static_handler))
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, Bytes};
    use axum::http::{Request, StatusCode};
    use axum::response::Response;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        // Call the health function directly
        let response = health().await;
        assert_eq!(response, "ok".to_string());
    }

    #[tokio::test]
    async fn test_health_route() {
        // Create the router
        let app = create_router();

        // Create a request to the health endpoint
        let request = Request::builder()
            .uri("/health")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        // Process the request
        let response = app.oneshot(request).await.unwrap();

        // Check the response status
        assert_eq!(response.status(), StatusCode::OK);

        // Check the response body
        let body = response_body_bytes(response).await;
        assert_eq!(&body[..], b"ok");
    }

    #[tokio::test]
    async fn test_not_found_route() {
        // Create the router
        let app = create_router();

        // Create a request to a non-existent endpoint
        let request = Request::builder()
            .uri("/non-existent")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        // Process the request
        let response = app.oneshot(request).await.unwrap();

        // Check the response status
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // Helper function to extract bytes from a response body
    async fn response_body_bytes(response: Response) -> Bytes {
        let body = response.into_body();
        // Use a reasonable size limit for the body (16MB)
        let bytes = axum::body::to_bytes(body, 16 * 1024 * 1024)
            .await
            .expect("Failed to read response body");
        bytes
    }
}
