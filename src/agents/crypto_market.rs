use tokio::process::Child;
use tracing;

use crate::utils::utils::run_agent;

pub async fn finance_query_agent(stream_id: &str, input: &str) -> Result<Child, String> {
    run_agent(stream_id, input, "./packages/genaiscript/genaisrc/finance-query.genai.mts").await
}


// #[cfg(test)]
// mod tests {
//     use std::fmt::Debug;
//     use crate::agents::search::search_agent;
//
//     #[tokio::test] // Mark the test function as async
//     async fn test_search_execution() {
//         let input = "Who won the 2024 presidential election?";
//
//         let mut command = search_agent("test-stream", input).await.unwrap();
//
//         // command.stdout.take().unwrap().read_to_string(&mut String::new()).await.unwrap();
//         // Optionally, you can capture and inspect stdout if needed:
//         let output = command.wait_with_output().await.expect("Failed to wait for output");
//         println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
//         println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
//     }
// }
