# open-web-agent-rs

A Rust-based web agent with an embedded OpenAI-compatible inference server (supports Gemma models only).

## Project Structure

This project is organized as a Cargo workspace with the following crates:

- `agent-server`: The main web agent server
- `local_inference_engine`: An embedded OpenAI-compatible inference server for Gemma models

## Setup

1. Clone the repository
2. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```
3. Install JavaScript dependencies:
   ```bash
   bun i
   ```
4. Start the SearXNG search engine:
   ```bash
   docker compose up -d searxng
   ```

## Running the Project

### Local Inference Engine

To run the local inference engine:

```bash
cd crates/local_inference_engine
cargo run --release -- --server
```

### Agent Server

To run the agent server:

```bash
cargo run -p agent-server
```

### Development Mode

For development with automatic reloading:

```bash
bun dev
```

## Building

To build all crates in the workspace:

```bash
cargo build
```

To build a specific crate:

```bash
cargo build -p agent-server
# or
cargo build -p local_inference_engine
```
