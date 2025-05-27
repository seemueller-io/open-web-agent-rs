# open-web-agent-rs
Remote genaiscript host for integration into conversational AI applications.
> This application is actively being ported, expect breaking changes.


## Quickstart
```bash
git clone <repo-url>
bun i
bun run build
bun dev
```

```javascript
#!/usr/bin/env node

(async () => {
    const url = "http://localhost:3006"
    const createAgentRequestParams = {
        
    };
    
    const createAgentRequest = fetch(url, {
        method: "POST",
        body: JSON.stringify(createAgentRequestParams)
    });
    
    const {streamId} = await createAgentRequest.json();
    
    
    const streamUrl = new URL(url);
    streamUrl.pathname = streamId;
    const eventsource = new EventSource(streamUrl)
    
    eventsource.onmessage = (event) => {
        console.log(JSON.stringify(event))
    }
})
```

### Disclaimer
This has not undergone a formal security assessment. You should do your own evaluation before using this.

### Features not included in this fork
- Capabilities API: Reports available agents via HTTP (useful for dynamic intent mapping)

### Planned Features
- Embed Model Context Protocol for client connectivity

## Documentation

Comprehensive documentation is available in the [docs](./docs) directory:

- [Installation Guide](./docs/installation.md) - How to install and set up the project
- [Configuration Guide](./docs/configuration.md) - Environment variables and configuration options
- [API Documentation](./docs/api.md) - API endpoints and usage examples
- [Authentication](./docs/tokens.md) - Authentication system documentation
- [Agents Guide](./docs/agents.md) - How to create and use agents
- [Input Documentation](./docs/input.md) - How input works for agents
- [Stream Data Format](./docs/streams.md) - How stream data is formatted for clients


### Setup
See [Installation](./docs/installation.md)

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

pub async fn agent(stream_id: &str, input: &str) -> Result<Child, String> {
    run_agent(stream_id, input, "./packages/genaiscript/genaisrc/your-agent.genai.mts").await
}
```

### 3. Register the Agent in the Module

Add your agent to the `src/agents/mod.rs` file:

```rust
pub(crate) mod your_module;
```

### 4. Register the Agent in the Webhook Handler

Add your agent to the match statement in the `use_agent` function in `src/handlers/agents.rs`:

```
// In the use_agent function
let cmd = match resource.as_str() {
    "web-search" => crate::agents::search::agent(agent_id.as_str(), &*input).await,
    "news-search" => crate::agents::news::agent(agent_id.as_str(), &*input).await,
    // Add your agent here
    "your-resource-name" => crate::agents::your_module::agent(agent_id.as_str(), &*input).await,
    _ => {
        tracing::error!("Unsupported resource type: {}", resource);
        return StatusCode::BAD_REQUEST.into_response();
    }
};
```

### 5. Configure Environment Variables

If your agent requires specific API keys or configuration, add them to the `ShimBinding` struct in `src/utils/utils.rs`.


### Fast Agent Development Workflow
1. Create script: create a new genaiscript script in `packages/genaiscript/genaisrc`
2. Setup a development executor: Map a package script in `package.json` to the script in step 1 following the existing examples
3. Iterate until agent is functional.
4. Follow the guide on adding a new agent to integrate it into the rust server.

## License

This project is licensed under the [MIT License](LICENSE)

## FAQ
> Q: Why Rust?
> A: Stronger primitives for STDIO and process management.


Development History (Nov 2024 – May 2025)
---

#### May 2025

* **Sanitize codebase and cleanup** *(2025-05-23)*

#### April 2025

* **Replace Perigon integration with SearxNG** *(2025-04-16)*
* **Enable authentication for SearxNG search** *(2025-04-04)*
* **Temporarily remove SearxNG password** *(2025-04-04)*
* **Deploy SearxNG search functionality** *(2025-04-01)*

#### March 2025

* **Deploy updated search functionality using SearxNG** *(2025-03-31)*
* **Resolve dependency issues and update Docker configuration** *(2025-03-31)*
* **Implement cryptocurrency market data fetching and quoting functionality** *(2025-03-20)*
* **Update AI model configuration** *(2025-03-20)*
* **Fix model provider issue** *(2025-03-18)*
* **Deploy configuration with auto-scaling capabilities (scales to zero)** *(2025-03-17)*

#### February 2025

* **Add image generation endpoint** *(2025-02-05)*

#### January 2025

* **Containerize application with Docker and deploy successfully** *(2025-01-27)*
* **Implement request call-count tracking and integrate tracing (tower-http)** *(2025-01-21)*
* **Disable caching mechanism** *(2025-01-16)*
* **Update deployment configuration to use GPT-4o-mini model** *(2025-01-15)*
* **Switch AI model provider back to OpenAI** *(2025-01-14)*

#### December 2024

* **Refactor database handling and web scraping logic** *(2024-12-30)*
* **Implement robust error handling and retry logic for webhooks** *(2024-12-29)*
* **Add sled database for persistent webhook handling** *(2024-12-28)*
* **Enhance scraping modules and build configurations** *(2024-12-28)*
* **Finalize URL reader implementation** *(2024-12-27)*
* **Upgrade news fetching mechanism and set specific search query provider** *(2024-12-21, 2024-12-18)*
* **Improve news search functionality (date filtering, formatting, error handling)** *(2024-12-16 to 2024-12-18)*
* **Add Perigon integration for news search** *(2024-12-16)*
* **Enhance VM resources and refine search result formatting** *(2024-12-16)*
* **Add stream activity tracking with reconnection handling** *(2024-12-15)*
* **Simplify AI search scripts and improve dependency management** *(2024-12-10)*
* **Update API keys and model configurations for better search reliability** *(2024-12-07, 2024-12-02)*

#### November 2024

* **Refactor project structure, enhance logging, and initial UI responses** *(2024-11-28)*
---
Note: Original commit history may be available by request.
