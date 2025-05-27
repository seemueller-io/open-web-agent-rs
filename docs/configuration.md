# Configuration Guide

## Environment Variables

open-web-agent-rs uses environment variables for configuration. These can be set in a `.env` file in the root directory or directly in your environment.

### Required Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `OPENAI_API_KEY` | API key for OpenAI services | `sk-...` |
| `BING_SEARCH_API_KEY` | API key for Bing Search | `...` |
| `TAVILY_API_KEY` | API key for Tavily | `tvly-...` |
| `GENAISCRIPT_MODEL_LARGE` | Large language model to use with GenAIScript | `gpt-4-turbo` |
| `GENAISCRIPT_MODEL_SMALL` | Small language model to use with GenAIScript | `gpt-3.5-turbo` |
| `SEARXNG_API_BASE_URL` | Base URL for SearxNG API | `https://searxng.example.com` |

### Optional Environment Variables

You may also want to configure these optional variables:

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `PORT` | Port for the server to listen on | `3006` | `8080` |
| `LOG_LEVEL` | Logging level | `info` | `debug`, `info`, `warn`, `error` |

## Docker Configuration

When running with Docker, you can pass environment variables using the `--env-file` flag or by setting them in the `compose.yml` file:

```yaml
services:
  web-agent:
    build:
      context: .
      dockerfile: Local.Dockerfile
    ports:
      - "3006:3006"
    environment:
      - OPENAI_API_KEY=${OPENAI_API_KEY}
      - BING_SEARCH_API_KEY=${BING_SEARCH_API_KEY}
      - TAVILY_API_KEY=${TAVILY_API_KEY}
      - GENAISCRIPT_MODEL_LARGE=${GENAISCRIPT_MODEL_LARGE}
      - GENAISCRIPT_MODEL_SMALL=${GENAISCRIPT_MODEL_SMALL}
      - SEARXNG_API_BASE_URL=${SEARXNG_API_BASE_URL}
```

## Authentication Configuration

The application uses FIPS204 signatures for authentication. See the [Authentication](./tokens.md) documentation for more details on configuring and using the authentication system.