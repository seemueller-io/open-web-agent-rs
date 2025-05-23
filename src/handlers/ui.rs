use axum::{
    body::Body,
    http::{StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;
use tracing::{debug, error};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub async fn serve_ui() -> impl IntoResponse {
    debug!("Serving UI request");

    // Attempt to retrieve the embedded "index.html"
    match Asset::get("index.html") {
        Some(content) => {
            debug!("Successfully retrieved index.html");
            Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "text/html")
                .body(Body::from(content.data))
                .unwrap()
        }
        None => {
            error!("index.html not found in embedded assets");
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 Not Found"))
                .unwrap()
        }
    }
}