use crate::utils::utils::run_agent;
use tokio::process::Child;

pub async fn scrape_agent(stream_id: &str, input: &str) -> Result<Child, String> {
    run_agent(stream_id, input, "./packages/genaiscript/genaisrc/web-scrape.genai.mts").await
}
