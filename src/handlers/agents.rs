use axum::response::Response;
use axum::{
    body::Body, extract::Path, extract::Query, http::StatusCode, response::IntoResponse, Json,
};
use bytes::Bytes;
use futures::stream::{Stream, StreamExt};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sled;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::Mutex;

// init sled
lazy_static! {
    static ref DB: Arc<Mutex<sled::Db>> = Arc::new(Mutex::new(
        sled::open("./open-web-agent-rs/db/stream_store").expect("Failed to open sled database")
    ));
}

pub async fn use_agent(Path(agent_id): Path<String>) -> impl IntoResponse {
    let db = DB.lock().await;
    match db.get(&agent_id) {
        Ok(Some(data)) => {
            let mut info: StreamInfo = match serde_json::from_slice(&data) {
                Ok(info) => info,
                Err(e) => {
                    tracing::error!("Failed to deserialize StreamInfo: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };

            // Increment the call_count in the database
            info.call_count += 1;
            let updated_info_bytes = match serde_json::to_vec(&info) {
                Ok(data) => data,
                Err(e) => {
                    tracing::error!("Failed to serialize updated StreamInfo: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };

            match db.insert(&agent_id, updated_info_bytes) {
                Ok(_) => {
                    if let Err(e) = db.flush_async().await {
                        tracing::error!(
                            "Failed to persist updated call_count to the database: {}",
                            e
                        );
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to update call_count in the database: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };

            let info: StreamInfo = match db.get(&agent_id) {
                Ok(Some(updated_data)) => match serde_json::from_slice(&updated_data) {
                    Ok(info) => info,
                    Err(e) => {
                        tracing::error!("Failed to deserialize updated StreamInfo: {}", e);
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                },
                Ok(None) => {
                    tracing::error!("Stream ID not found after update: {}", agent_id);
                    return StatusCode::NOT_FOUND.into_response();
                }
                Err(e) => {
                    tracing::error!("Failed to fetch updated record from DB: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };

            if (info.call_count > 1) {
                return StatusCode::OK.into_response();
            }

            let resource = info.resource;
            let input = serde_json::to_string(&info.payload.input).unwrap_or_default();

            tracing::debug!(
                "Processing webhook - Resource: {}, Stream ID: {}",
                resource,
                agent_id
            );

            let cmd = match resource.as_str() {
                "web-search" => crate::agents::search::agent(agent_id.as_str(), &*input).await,
                "news-search" => crate::agents::news::agent(agent_id.as_str(), &*input).await,
                "image-generator" => {
                    crate::agents::image_generator::agent(agent_id.as_str(), &*input).await
                }
                "web-scrape" => crate::agents::scrape::agent(agent_id.as_str(), &*input).await,
                _ => {
                    tracing::error!("Unsupported resource type: {}", resource);
                    return StatusCode::BAD_REQUEST.into_response();
                }
            };

            let mut cmd = match cmd {
                Ok(cmd) => cmd,
                Err(e) => {
                    tracing::error!("Agent execution failed: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };

            let stdout = match cmd.stdout.take() {
                Some(stdout) => stdout,
                None => {
                    tracing::error!("No stdout available for the command.");
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };

            let reader = BufReader::new(stdout);
            let sse_stream = reader_to_stream(reader, agent_id.clone());

            return Response::builder()
                .header("Content-Type", "text/event-stream")
                .header("Cache-Control", "no-cache, no-transform")
                .header("Connection", "keep-alive")
                .header("X-Accel-Buffering", "yes")
                .body(Body::from_stream(sse_stream))
                .unwrap();
        }
        Ok(None) => {
            tracing::error!("Stream ID not found: {}", agent_id);
            StatusCode::NOT_FOUND.into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch from DB: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

fn reader_to_stream<R>(
    reader: BufReader<R>,
    stream_id: String,
) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>
where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
{
    let stream = futures::stream::unfold(reader, move |mut reader| async move {
        let mut line = String::new();
        match reader.read_line(&mut line).await {
            Ok(0) => None,
            Ok(_) => Some((
                Ok(Bytes::from(format!("data: {}\n\n", line.trim()))),
                reader,
            )),
            Err(e) => Some((Err(e), reader)),
        }
    });

    let stream_with_done = stream.chain(futures::stream::once(async {
        Ok(Bytes::from("data: [DONE]\n\n"))
    }));

    Box::pin(stream_with_done)
}

#[derive(Deserialize, Serialize, Debug)]
struct Payload {
    input: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct StreamInfo {
    resource: String,
    payload: Payload,
    parent: String,
    call_count: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WebhookPostRequest {
    id: String,
    resource: String,
    payload: Payload,
    parent: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct WebhookPostResponse {
    stream_url: String,
}

pub async fn create_agent(Json(payload): Json<WebhookPostRequest>) -> impl IntoResponse {
    let db = DB.lock().await;

    tracing::info!("Received webhook post request with ID: {}", payload.id);

    let stream_id = payload.id.clone();
    let info = StreamInfo {
        resource: payload.resource.clone(),
        payload: payload.payload,
        parent: payload.parent.clone(),
        call_count: 0,
    };

    let info_bytes = match serde_json::to_vec(&info) {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("Failed to serialize StreamInfo: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // Use atomic compare_and_swap operation
    match db.compare_and_swap(
        &stream_id,
        None as Option<&[u8]>,
        Some(info_bytes.as_slice()),
    ) {
        Ok(_) => {
            // Force an immediate sync to disk
            match db.flush_async().await {
                Ok(_) => {
                    // Verify the write by attempting to read it back
                    match db.get(&stream_id) {
                        Ok(Some(_)) => {
                            let stream_url = format!("/webhooks/{}", stream_id);
                            tracing::info!(
                                "Successfully created and verified stream URL: {}",
                                stream_url
                            );
                            Json(WebhookPostResponse { stream_url }).into_response()
                        }
                        Ok(None) => {
                            tracing::error!("Failed to verify stream creation: {}", stream_id);
                            StatusCode::INTERNAL_SERVER_ERROR.into_response()
                        }
                        Err(e) => {
                            tracing::error!("Error verifying stream creation: {}", e);
                            StatusCode::INTERNAL_SERVER_ERROR.into_response()
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to flush DB: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to insert stream info: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
