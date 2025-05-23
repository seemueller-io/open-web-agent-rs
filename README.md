# web-agent-rs
Remote genaiscript host for integration into conversational AI applications.
> This project is actively being developed to suit more use-cases, expect breaking changes.


### Setup
See [Installation](./docs/installation.md)


### Disclaimer
This has not undergone a formal security assessment. You should do your own evaluation before using this.

### How it works
1. A chat client specifies the URL to this host in their environment.
2. They send a request with their credentials to create a stream resource 

## Adding New Agents

This project allows you to create and integrate new agents that can perform various tasks. Here's how to add a new agent:

### 1. Create a GenAIScript File

Create a new `.genai.mts` file in the `packages/genaiscript/genaisrc/` directory. This file will contain the agent's logic.

Example structure of a GenAIScript file:

```typescript
import {SomeClient} from "@agentic/some-package";
import "./tools/some-tool.genai.mjs"

script({
    title: "your_agent_name",
    maxTokens: 8192,
    cache: false,
    tools: ["tool-name"],
});

def("USER_INPUT", env.vars.user_input);

$`You are an assistant that performs a specific task.
- Instruction 1
- Instruction 2
- Instruction 3`
```

### 2. Create a Rust Agent Function

Create a new Rust file in the `src/agents/` directory or add a function to an existing file. This function will be a wrapper that calls the GenAIScript file.

Example agent function:

```rust
use tokio::process::Child;
use tracing;

use crate::utils::utils::run_agent;

pub async fn your_agent_name(stream_id: &str, input: &str) -> Result<Child, String> {
    run_agent(stream_id, input, "./packages/genaiscript/genaisrc/your-agent.genai.mts").await
}
```

### 3. Register the Agent in the Module

Add your agent to the `src/agents/mod.rs` file:

```rust
pub mod your_agent_name;
```

### 4. Register the Agent in the Webhook Handler

Add your agent to the match statement in the `handle_webhooks` function in `src/handlers/webhooks.rs`:

```
// In the handle_webhooks function
let cmd = match resource.as_str() {
    "web-search" => search_agent(stream_id.as_str(), &*input).await,
    "news-search" => news_agent(stream_id.as_str(), &*input).await,
    // Add your agent here
    "your-resource-name" => your_agent_name(stream_id.as_str(), &*input).await,
    _ => {
        tracing::error!("Unsupported resource type: {}", resource);
        return StatusCode::BAD_REQUEST.into_response();
    }
};
```

### 5. Configure Environment Variables

If your agent requires specific API keys or configuration, add them to the `ShimBinding` struct in `src/utils/utils.rs`.

## Existing Agents

The project currently includes the following agents:

- **Web Search**: Performs web searches using SearxNG
- **News Search**: Searches for news articles
- **Image Generator**: Generates images based on text prompts
- **Finance Query**: Provides financial information
- **Web Scrape**: Scrapes content from web pages

## Documentation

Comprehensive documentation is available in the [docs](./docs) directory:

- [Installation Guide](./docs/installation.md) - How to install and set up the project
- [Configuration Guide](./docs/configuration.md) - Environment variables and configuration options
- [API Documentation](./docs/api.md) - API endpoints and usage examples
- [Authentication](./docs/tokens.md) - Authentication system documentation
- [Agents Guide](./docs/agents.md) - How to create and use agents

## License

This project is licensed under the [MIT License](LICENSE)
