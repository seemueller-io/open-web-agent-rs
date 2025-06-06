use crate::utils::utils::run_agent;
use tokio::process::Child;

pub async fn agent(stream_id: &str, input: &str) -> Result<Child, String> {
    tracing::debug!(
                "Running image generator, \ninput: {}",
                input
            );
    run_agent(stream_id, input, "./packages/genaiscript/genaisrc/image-generator.genai.mts", 10).await
}
