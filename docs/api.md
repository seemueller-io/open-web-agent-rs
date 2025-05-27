# API Documentation

## Authentication

All API endpoints require authentication using a session token. The token should be included in the `Authorization` header as a Bearer token:

```
Authorization: Bearer <session_token>
```

For more information on authentication and token generation, see the [Authentication](./tokens.md) documentation.

## Endpoints

### Health Check

```
GET /health
```

Returns a simple "ok" response to indicate that the server is running.

**Response:**
```
200 OK
ok
```

### Create Stream Resource

```
POST /api/webhooks
```

Creates a new stream resource for an agent.

**Request Body:**
```json
{
  "resource": "web-search",
  "input": "What is the capital of France?"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `resource` | string | The type of agent to use (e.g., "web-search", "news-search", "image-generator", "web-scrape") |
| `input` | string | The input query or prompt for the agent |

**Response:**
```json
{
  "stream_id": "abc123"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `stream_id` | string | The ID of the created stream resource |

### Consume Stream Resource

```
GET /webhooks/:stream_id
```

Consumes a stream resource, returning the agent's response as a server-sent event stream.

**Path Parameters:**
- `stream_id`: The ID of the stream resource to consume

**Response:**
Server-sent event stream with the agent's response.

## Available Agents

The following agent types are available for use with the `resource` field in the Create Stream Resource endpoint:

| Agent Type | Description |
|------------|-------------|
| `web-search` | Performs web searches using SearxNG |
| `news-search` | Searches for news articles |
| `image-generator` | Generates images based on text prompts |
| `web-scrape` | Scrapes content from web pages |

## Error Responses

| Status Code | Description |
|-------------|-------------|
| 400 | Bad Request - The request was malformed or missing required fields |
| 401 | Unauthorized - Authentication failed or token is invalid |
| 404 | Not Found - The requested resource was not found |
| 500 | Internal Server Error - An unexpected error occurred |

## Example Usage

### Creating a Stream Resource

```bash
curl -X POST https://your-server.com/api/webhooks \
  -H "Authorization: Bearer <session_token>" \
  -H "Content-Type: application/json" \
  -d '{"resource": "web-search", "input": "What is the capital of France?"}'
```

### Consuming a Stream Resource

```bash
curl https://your-server.com/webhooks/abc123 \
  -H "Authorization: Bearer <session_token>"
```
