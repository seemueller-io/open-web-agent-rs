pub(crate) mod news;
pub(crate) mod scrape;
pub(crate) mod search;
pub(crate) mod image_generator;
pub(crate) mod deep_research;

use std::sync::Arc;

use rmcp::{
    Error as McpError, RoleServer, ServerHandler, const_string, model::*,
    service::RequestContext, tool,
};
use tokio::process::Child;

#[derive(Clone)]
pub struct Agents;

#[tool(tool_box)]
impl Agents {
    pub fn new() -> Self {
        Self {}
    }

    #[tool(description = "Search the web for information")]
    async fn search(
        &self,
        #[tool(param)]
        #[schemars(description = "The search query")]
        query: String,
    ) -> Result<CallToolResult, McpError> {
        match search::agent("tool-search", &query).await {
            Ok(child) => handle_agent_result(child).await,
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }

    #[tool(description = "Search for news articles")]
    async fn news(
        &self,
        #[tool(param)]
        #[schemars(description = "The news search query")]
        query: String,
    ) -> Result<CallToolResult, McpError> {
        match news::agent("tool-news", &query).await {
            Ok(child) => handle_agent_result(child).await,
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }

    #[tool(description = "Scrape content from a webpage")]
    async fn scrape(
        &self,
        #[tool(param)]
        #[schemars(description = "The URL to scrape")]
        url: String,
    ) -> Result<CallToolResult, McpError> {
        match scrape::agent("tool-scrape", &url).await {
            Ok(child) => handle_agent_result(child).await,
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }

    #[tool(description = "Generate an image based on a description")]
    async fn generate_image(
        &self,
        #[tool(param)]
        #[schemars(description = "The image description")]
        description: String,
    ) -> Result<CallToolResult, McpError> {
        match image_generator::agent("tool-image", &description).await {
            Ok(child) => handle_agent_result(child).await,
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }

    #[tool(description = "Perform deep research on a topic")]
    async fn deep_research(
        &self,
        #[tool(param)]
        #[schemars(description = "The research topic")]
        topic: String,
    ) -> Result<CallToolResult, McpError> {
        match deep_research::agent("tool-research", &topic).await {
            Ok(child) => handle_agent_result(child).await,
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for Agents {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides various agent tools for web search, news search, web scraping, image generation, and deep research.".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            tracing::info!(?initialize_headers, %initialize_uri, "initialize from http server");
        }
        Ok(self.get_info())
    }
}

async fn handle_agent_result(mut child: Child) -> Result<CallToolResult, McpError> {
    use tokio::io::AsyncReadExt;

    let output = match child.wait_with_output().await {
        Ok(output) => output,
        Err(e) => return Err(McpError::internal_error(format!("Failed to get agent output: {}", e), None)),
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(McpError::internal_error(
            format!("Agent failed with status {}: {}", output.status, stderr),
            None,
        ));
    }

    Ok(CallToolResult::success(vec![Content::text(stdout)]))
}
