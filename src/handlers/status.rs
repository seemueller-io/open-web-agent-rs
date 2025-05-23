// src/handlers/status.rs
pub async fn handle_status() -> &'static str {
    tracing::debug!("Status check requested");
    "Server is running"
}