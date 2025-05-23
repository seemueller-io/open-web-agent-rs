// src/config.rs
pub struct AppConfig {
    pub env_vars: Vec<String>,
}


impl AppConfig {
    pub fn new() -> Self {
        // Load .env file if it exists
        match dotenv::dotenv() {
            Ok(_) => tracing::debug!("Loaded .env file successfully"),
            Err(e) => tracing::debug!("No .env file found or error loading it: {}", e),
        }

        Self {
            env_vars: vec![
                "OPENAI_API_KEY".to_string(),
                "BING_SEARCH_API_KEY".to_string(),
                "TAVILY_API_KEY".to_string(),
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