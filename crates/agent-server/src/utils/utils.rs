// utils.rs
use tokio::process::{Child, Command}; // Use tokio::process::Child and Command
use std::env;
use tokio::time::{timeout, Duration};
use tracing;


pub struct ShimBinding {
    user_input: String,
    file_path: String,
    openai_api_key: String,
    openai_api_base: String,
    genaiscript_model_large: String,
    genaiscript_model_small: String,
    searxng_api_base_url: String,
    searxng_password: String,
}

impl ShimBinding {
    pub fn new(user_input: String, file_path: String) -> Self { 
        Self {
            user_input,
            file_path, // Initialize the new field
            openai_api_key: env::var("OPENAI_API_KEY").unwrap_or_default(),
            openai_api_base: env::var("OPENAI_API_BASE").unwrap_or_default(),
            genaiscript_model_large: env::var("GENAISCRIPT_MODEL_LARGE").unwrap_or_default(),
            genaiscript_model_small: env::var("GENAISCRIPT_MODEL_SMALL").unwrap_or_default(),
            searxng_api_base_url: env::var("SEARXNG_API_BASE_URL").unwrap_or_default(),
            searxng_password: env::var("SEARXNG_PASSWORD").unwrap_or_default(),
        }
    }

    pub fn execute(&self) -> std::io::Result<Child> {
        let mut command = Command::new("./dist/genaiscript-rust-shim.js");
        command
            .arg("--file")
            .arg(&self.file_path) // Use the file_path field instead of hardcoded value
            .arg(format!("USER_INPUT={}", self.user_input))
            .env("OPENAI_API_KEY", &self.openai_api_key)
            .env("OPENAI_API_BASE", &self.openai_api_base)
            .env("GENAISCRIPT_MODEL_LARGE", &self.genaiscript_model_large)
            .env("GENAISCRIPT_MODEL_SMALL", &self.genaiscript_model_small)
            .env("SEARXNG_API_BASE_URL", &self.searxng_api_base_url)
            .env("SEARXNG_PASSWORD", &self.searxng_password)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit());

        command.spawn()
    }
}


 /// wrapper executes an agent with a timeout
pub async fn run_agent(stream_id: &str, input: &str, file_path: &str, timeout_seconds: u64 ) -> Result<Child, String> {
    tracing::debug!("Initiating agent for stream {} with file path {}", stream_id, file_path);

    let shim_binding = ShimBinding::new(input.to_string(), file_path.to_string());
    let spawn_future = async move {
        match shim_binding.execute() {
            Ok(child) => Ok(child),
            Err(e) => {
                tracing::error!("Failed to spawn shim process: {}", e);
                Err(e.to_string())
            }
        }
    };

    timeout(Duration::from_secs(timeout_seconds), spawn_future)
        .await
        .unwrap_or_else(|_| Err("Command timed out after 10 seconds".to_string()))
}
