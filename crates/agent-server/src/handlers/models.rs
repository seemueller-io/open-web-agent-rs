use axum::{
    extract::Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Debug)]
pub struct ModelsResponse {
    object: String,
    data: Vec<Model>,
}

#[derive(Serialize, Debug)]
pub struct Model {
    id: String,
    object: String,
    created: u64,
    owned_by: String,
}

pub async fn list_models() -> impl IntoResponse {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Create a response with a default model
    let response = ModelsResponse {
        object: "list".to_string(),
        data: vec![
            Model {
                id: "gpt-3.5-turbo".to_string(),
                object: "model".to_string(),
                created: current_time,
                owned_by: "open-web-agent-rs".to_string(),
            },
            Model {
                id: "gpt-4".to_string(),
                object: "model".to_string(),
                created: current_time,
                owned_by: "open-web-agent-rs".to_string(),
            },
        ],
    };

    Json(response)
}