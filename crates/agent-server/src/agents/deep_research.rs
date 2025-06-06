use crate::utils::utils::run_agent;
use tokio::process::Child;
use tracing;

pub async fn agent(stream_id: &str, input: &str) -> Result<Child, String> {
    run_agent(stream_id, input, "./packages/genaiscript/genaisrc/deep-research.genai.mts", 60).await
}


#[cfg(test)]
mod tests {
    use crate::agents::deep_research::agent;
    use std::fmt::Debug;

    #[tokio::test]
    async fn test_deepresearch() {
        // a really provocative question for research that generally yields infinite complexity with each run
        let input = "What is a life of meaning?";

        let mut command = agent("test-deepresearch-agent", input).await.unwrap();

        // let mut stdout = String::new();

        // command.stdout.take().unwrap().read_to_string(&mut stdout).await.unwrap();

        // println!("stdout: {}", stdout);
        // // Optionally, you can capture and inspect stdout if needed:
        let _output = command.wait_with_output().await.expect("Failed to wait for output");
        // println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        // println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
} 
