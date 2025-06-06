# open-web-agent-rs

A Rust-based web agent with an embedded openai compatible inference server (supports gemma models only).

## Quickstart
```bash
cp .env.example .env
bun i
(cd local_inference_server && cargo run --release -- --server)
docker compose up -d searxng
bun dev
```
