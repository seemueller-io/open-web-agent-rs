use axum::response::Response;
use axum::{
    body::Body, extract::Json, http::StatusCode, response::IntoResponse,
};
use bytes::Bytes;
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::pin::Pin;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::utils::utils::run_agent;

// Custom function to format streaming responses according to OpenAI API format
pub fn openai_stream_format<R>(
    reader: BufReader<R>,
    request_id: String,
    model: String,
) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>
where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
{
    let stream = futures::stream::unfold((reader, 0), move |(mut reader, index)| {
        let request_id = request_id.clone();
        let model = model.clone();
        async move {
            let mut line = String::new();
            match reader.read_line(&mut line).await {
                Ok(0) => None,
                Ok(_) => {
                    let content = line.trim();
                    // Skip empty lines
                    if content.is_empty() {
                        return Some((Ok(Bytes::from("")), (reader, index)));
                    }

                    // Format as OpenAI API streaming response
                    let chunk = serde_json::json!({
                        "id": format!("chatcmpl-{}", request_id),
                        "object": "chat.completion.chunk",
                        "created": std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                        "model": model,
                        "choices": [{
                            "index": index,
                            "delta": {
                                "content": content
                            },
                            "finish_reason": null
                        }]
                    });

                    Some((
                        Ok(Bytes::from(format!("data: {}\n\n", chunk.to_string()))),
                        (reader, index),
                    ))
                }
                Err(e) => Some((Err(e), (reader, index))),
            }
        }
    });

    // Add the [DONE] message at the end
    let stream_with_done = stream.filter(|result| {
        futures::future::ready(match result {
            Ok(bytes) => !bytes.is_empty(),
            Err(_) => true,
        })
    }).chain(futures::stream::once(async {
        Ok(Bytes::from("data: [DONE]\n\n"))
    }));

    Box::pin(stream_with_done)
}

#[derive(Deserialize, Debug)]
pub struct ModelContextRequest {
    messages: Vec<Message>,
    model: Option<String>,
    stream: Option<bool>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
pub struct ModelContextResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Serialize, Debug)]
pub struct Choice {
    index: u32,
    message: Message,
    finish_reason: String,
}

pub async fn model_context(
    headers: axum::http::HeaderMap,
    Json(payload): Json<ModelContextRequest>
) -> impl IntoResponse {
    // Generate a unique ID for this request
    let request_id = uuid::Uuid::new_v4().to_string();

    // Convert messages to a format that can be passed to the agent
    let input = serde_json::to_string(&payload.messages).unwrap_or_default();

    // Use the web-search agent for now, but this could be customized based on the model parameter
    let agent_file = "./packages/genaiscript/genaisrc/web-search.genai.mts";

    tracing::debug!(
        "Executing model context request - Id: {}",
        request_id
    );

    // Default timeout of 60 seconds
    let mut cmd = match run_agent(&request_id, &input, agent_file, 60).await {
        Ok(cmd) => cmd,
        Err(e) => {
            tracing::error!("Model context execution failed: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // Check if streaming is requested either via the stream parameter or Accept header
    let accept_header = headers.get("accept").and_then(|h| h.to_str().ok()).unwrap_or("");
    let is_streaming = payload.stream.unwrap_or(false) || accept_header.contains("text/event-stream");

    // If streaming is requested, return a streaming response
    if is_streaming {
        let stdout = match cmd.stdout.take() {
            Some(stdout) => stdout,
            None => {
                tracing::error!("No stdout available for the command.");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        let reader = BufReader::new(stdout);
        let model = payload.model.clone().unwrap_or_else(|| "default-model".to_string());
        let sse_stream = openai_stream_format(reader, request_id.clone(), model);

        return Response::builder()
            .header("Content-Type", "text/event-stream")
            .header("Cache-Control", "no-cache, no-transform")
            .header("Connection", "keep-alive")
            .header("X-Accel-Buffering", "yes")
            .body(Body::from_stream(sse_stream))
            .unwrap();
    } else {
        // For non-streaming responses, we need to collect all output and return it as a single response
        // This is a simplified implementation and might need to be adjusted based on actual requirements
        let response = ModelContextResponse {
            id: format!("chatcmpl-{}", request_id),
            object: "chat.completion".to_string(),
            created: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            model: payload.model.unwrap_or_else(|| "default-model".to_string()),
            choices: vec![Choice {
                index: 0,
                message: Message {
                    role: "assistant".to_string(),
                    content: "This is a placeholder response. The actual implementation would process the agent's output.".to_string(),
                },
                finish_reason: "stop".to_string(),
            }],
        };

        return Json(response).into_response();
    }
}
