pub struct Runtime {
    pub env_vars: Vec<String>,
}


impl Runtime {
    pub fn configure() -> Self {
        // automatic configuration between local/docker environments
        match dotenv::dotenv() {
            Ok(_) => tracing::debug!("Loaded .env file successfully"),
            Err(e) => tracing::debug!("No .env file found or error loading it: {}", e),
        }

        Self {
            env_vars: vec![
                "OPENAI_API_KEY".to_string(),
                "GENAISCRIPT_MODEL_LARGE".to_string(),
                "GENAISCRIPT_MODEL_SMALL".to_string(),
                "SEARXNG_API_BASE_URL".to_string(),
            ],
        }
    }

    pub fn get_env_var(&self, key: &str) -> String {
        std::env::var(key).unwrap_or_default()
    }
}