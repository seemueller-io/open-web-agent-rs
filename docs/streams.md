# Stream Data Format

This document describes how the stream data is formatted as it comes across the wire to the client.

## Overview

The open-web-agent-rs uses Server-Sent Events (SSE) to stream data from agents to clients. This allows for real-time updates as the agent processes the request and generates responses.

## Stream Format

When you consume a stream resource by making a GET request to `/webhooks/:stream_id`, the server responds with a stream of data in the SSE format. Each piece of data from the agent is sent as an SSE event with the following format:

```
data: <content>\n\n
```

Where `<content>` is a line of output from the agent.

### Stream Completion

When the agent has finished processing and there is no more data to send, the server sends a final event to indicate the stream has completed:

```
data: [DONE]\n\n
```

This allows clients to know when the stream has ended and they can stop listening for events.

## HTTP Headers

The server includes the following HTTP headers in the response:

- `Content-Type: text/event-stream` - Indicates that the response is an SSE stream
- `Cache-Control: no-cache, no-transform` - Prevents caching of the stream
- `Connection: keep-alive` - Keeps the connection open for the duration of the stream
- `X-Accel-Buffering: yes` - Controls buffering behavior for certain proxies

## Client-Side Handling

Clients should use an EventSource or similar mechanism to consume the SSE stream. Here's an example of how to consume the stream using JavaScript:

```javascript
const eventSource = new EventSource('/webhooks/your-stream-id', {
  headers: {
    'Authorization': 'Bearer your-session-token'
  }
});

eventSource.onmessage = (event) => {
  if (event.data === '[DONE]') {
    // Stream is complete, close the connection
    eventSource.close();
    return;
  }
  
  // Process the data
  console.log('Received data:', event.data);
};

eventSource.onerror = (error) => {
  console.error('EventSource error:', error);
  eventSource.close();
};
```

## Data Content

The content of each data event depends on the specific agent being used. For example:
- Web search agents may return search results and snippets
- News search agents may return article headlines and summaries
- Image generator agents may return image URLs or base64-encoded images

Refer to the specific agent documentation for details on the format of the data they return.