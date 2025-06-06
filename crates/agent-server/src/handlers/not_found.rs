use axum::{
    http::StatusCode,
    Json,
    response::IntoResponse,
};

pub async fn handle_not_found() -> impl IntoResponse {
    tracing::warn!("404 Not Found error occurred");

    let error_response = serde_json::json!({
        "error": "Route Not Found",
        "status": 404
    });

    (StatusCode::NOT_FOUND, Json(error_response))
}