# Installation Guide

## Prerequisites

Before installing web-agent-rs, ensure you have the following prerequisites:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Node.js](https://nodejs.org/) (for GenAIScript)
- [Bun](https://bun.sh/) (for package management)
- [Docker](https://www.docker.com/get-started) (optional, for containerized deployment)

## Environment Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/seemueller-io/open-web-agent-rs.git
   cd open-web-agent-rs
   ```

2. Create a `.env` file in the root directory with the following variables:
   ```
   OPENAI_API_KEY=your_openai_api_key
   BING_SEARCH_API_KEY=your_bing_search_api_key
   TAVILY_API_KEY=your_tavily_api_key
   GENAISCRIPT_MODEL_LARGE=gpt-4-turbo
   GENAISCRIPT_MODEL_SMALL=gpt-3.5-turbo
   SEARXNG_API_BASE_URL=your_searxng_url
   ```

## Local Development

1. Install Rust dependencies:
   ```bash
   cargo build
   ```

2. Install JavaScript dependencies:
   ```bash
   bun install
   ```

3. Run the server:
   ```bash
   cargo run
   ```

   The server will start on `http://localhost:3006`.

## Docker Deployment

You can also run the application using Docker:

1. Build the Docker image:
   ```bash
   docker build -t web-agent-rs -f Dockerfile .
   ```

2. Run the container:
   ```bash
   docker run -p 3006:3006 --env-file .env web-agent-rs
   ```

Alternatively, you can use Docker Compose:

```bash
docker-compose up
```

## Configuration Options

The application can be configured using environment variables. See the [Configuration](./configuration.md) documentation for more details.