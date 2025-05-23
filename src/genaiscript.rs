use std::collections::HashMap;
use std::path::PathBuf;
use tokio::process::{Child, Command};
use tracing;

const DEFAULT_ENV_VARS: [&str; 4] = [
    "OPENAI_API_KEY",
    "OPENAI_API_BASE",
    "GENAISCRIPT_MODEL_LARGE",
    "GENAISCRIPT_MODEL_SMALL",
];

pub struct GenAIScriptConfig {
    script_path: PathBuf,
    output_dir: PathBuf,
    stream_id: String,
    user_input: String,
    retry_count: u32,
    env_vars: HashMap<String, String>,
}

impl GenAIScriptConfig {
    pub fn new(script_path: impl Into<PathBuf>, stream_id: impl Into<String>, user_input: impl Into<String>) -> Self {
        let mut env_vars = HashMap::new();

        // Initialize with default environment variables
        for var in DEFAULT_ENV_VARS {
            if let Ok(value) = std::env::var(var) {
                env_vars.insert(var.to_string(), value);
            }
        }

        Self {
            script_path: script_path.into(),
            output_dir: PathBuf::from("./web-agent-rs/output"),
            stream_id: stream_id.into(),
            user_input: user_input.into(),
            retry_count: 0,
            env_vars,
        }
    }

    pub fn with_output_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.output_dir = dir.into();
        self
    }

    pub fn with_retry_count(mut self, count: u32) -> Self {
        self.retry_count = count;
        self
    }

    pub fn with_additional_env_vars(mut self, vars: HashMap<String, String>) -> Self {
        self.env_vars.extend(vars);
        self
    }
}

pub async fn run_genaiscript(config: GenAIScriptConfig) -> Result<Child, String> {
    tracing::debug!("Initiating GenAIScript for stream {}", config.stream_id);

    let output_path = config.output_dir.join(&config.stream_id);

    let mut command = Command::new("bunx");
    command
        .arg("genaiscript")
        .arg("run")
        .arg(&config.script_path)
        // .arg("--fail-on-errors")
        .arg("—out-trace")
        .arg(output_path)
        .arg("--retry")
        .arg(config.retry_count.to_string())
        .arg("--vars")
        .arg(format!("USER_INPUT='{}'", config.user_input));

    // Add environment variables
    for (key, value) in config.env_vars {
        command.env(key, value);
    }

    command
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .map_err(|e| {
            tracing::error!("Failed to spawn genaiscript process: {}", e);
            e.to_string()
        })
}