# Agents Documentation

## Overview

Agents are the core components of web-agent-rs that perform specific tasks. Each agent is implemented as a GenAIScript file that defines its behavior and a Rust function that wraps the script.

## Available Agents

The following agents are currently available:

| Agent Type | Description | Resource Name |
|------------|-------------|---------------|
| Web Search | Performs web searches using SearxNG | `web-search` |
| News Search | Searches for news articles | `news-search` |
| Image Generator | Generates images based on text prompts | `image-generator` |
| Finance Query | Provides financial information | `finance-query` |
| Web Scrape | Scrapes content from web pages | `web-scrape` |

## Creating a New Agent

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

Add your agent to the match statement in the `handle_webhooks_post` function in `src/handlers/webhooks.rs`:

```
// In the handle_webhooks_post function
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

## Agent Tools

Agents can use various tools to perform their tasks. These tools are defined in the `packages/genaiscript/genaisrc/tools/` directory.

To use a tool in your agent, import it in your GenAIScript file:

```typescript
import "./tools/some-tool.genai.mjs"
```

And add it to the `tools` array in the `script` function:

```typescript
script({
    title: "your_agent_name",
    maxTokens: 8192,
    cache: false,
    tools: ["tool-name"],
});
```

## Testing Agents

You can test your agent by sending a request to the API:

```bash
curl -X POST https://your-server.com/api/webhooks \
  -H "Authorization: Bearer <session_token>" \
  -H "Content-Type: application/json" \
  -d '{"resource": "your-resource-name", "input": "Your test input"}'
```

Then consume the stream to see the agent's response:

```bash
curl https://your-server.com/webhooks/<stream_id> \
  -H "Authorization: Bearer <session_token>"
```

## Best Practices

1. Keep your GenAIScript files focused on a single task
2. Use appropriate tools for the task
3. Handle errors gracefully
4. Provide clear instructions in the agent's prompt
5. Test your agent thoroughly before deploying

## Related Documentation

- [Input Documentation](./input.md) - How input works for agents
- [API Documentation](./api.md) - API endpoints and usage examples
