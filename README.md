# open-web-agent-rs

A Rust-based web agent with local inference capabilities.

## Components

### Local Inference Engine

The [Local Inference Engine](./local_inference_engine/README.md) provides a way to run large language models locally. It supports both CLI mode for direct text generation and server mode with an OpenAI-compatible API.

Features:
- Run Gemma models locally (1B, 2B, 7B, 9B variants)
- CLI mode for direct text generation
- Server mode with OpenAI-compatible API
- Support for various model configurations (base, instruction-tuned)
- Metal acceleration on macOS

See the [Local Inference Engine README](./local_inference_engine/README.md) for detailed usage instructions.

### Web Server

Server is being converted to MCP. Things are probably broken.

```text
bun i
bun dev
```
