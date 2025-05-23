use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use futures::StreamExt;
use tokio_util::io::ReaderStream;

pub async fn handle_stream() -> impl IntoResponse {
    use tokio::process::Command;

    let user_input = "Who won the 2024 election?";
    tracing::debug!("Handling stream request with input: {}", user_input);

    // Check environment variables
    for env_var in ["OPENAI_API_KEY", "BING_SEARCH_API_KEY", "TAVILY_API_KEY"] {
        if std::env::var(env_var).is_ok() {
            tracing::debug!("{} is set", env_var);
        } else {
            tracing::warn!("{} is not set", env_var);
        }
    }

    let mut cmd = match Command::new("genaiscript")
        .arg("run")
        .arg("genaisrc/web-search.genai.mts")
        .arg("--vars")
        .arg(format!("USER_INPUT='{}'", user_input))
        .env("OPENAI_API_KEY", std::env::var("OPENAI_API_KEY").unwrap_or_default())
        .env("BING_SEARCH_API_KEY", std::env::var("BING_SEARCH_API_KEY").unwrap_or_default())
        .env("TAVILY_API_KEY", std::env::var("TAVILY_API_KEY").unwrap_or_default())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn() {
        Ok(cmd) => {
            tracing::debug!("Successfully spawned genaiscript process");
            cmd
        }
        Err(e) => {
            tracing::error!("Failed to spawn genaiscript process: {}", e);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to start process"))
                .unwrap();
        }
    };

    let stdout = match cmd.stdout.take() {
        Some(stdout) => {
            tracing::debug!("Successfully captured stdout from process");
            stdout
        }
        None => {
            tracing::error!("Failed to capture stdout from process");
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to capture process output"))
                .unwrap();
        }
    };

    let reader = tokio::io::BufReader::new(stdout);
    let stream = ReaderStream::new(reader);
    let mapped_stream = stream.map(|r| {
        match r {
            Ok(bytes) => {
                tracing::trace!("Received {} bytes from stream", bytes.len());
                Ok(bytes)
            }
            Err(e) => {
                tracing::error!("Error reading from stream: {}", e);
                Err(e)
            }
        }
    });

    tracing::debug!("Setting up SSE response");
    Response::builder()
        .header("Content-Type", "text/event-stream")
        .body(Body::from_stream(mapped_stream))
        .unwrap()
}