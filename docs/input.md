# Agent Input Documentation

## Overview

This document explains how input works for agents in the web-agent-rs project. Understanding how input is processed is essential for creating effective agents and integrating them with client applications.

## Input Flow

The input for agents follows this flow:

1. **API Request**: A client sends a request to the API with a resource type and input string.
2. **Webhook Handler**: The webhook handler extracts the input and passes it to the appropriate agent function.
3. **Agent Function**: The agent function passes the input to the `run_agent` utility function.
4. **ShimBinding**: The `run_agent` function creates a `ShimBinding` that passes the input to the GenAIScript shim as a command-line argument.
5. **GenAIScript**: The GenAIScript accesses the input through `env.vars.user_input` and typically assigns it to a variable named "USER_INPUT".

## Input Format

The input is a simple string that can contain any text. There are no specific format requirements, but the input should be relevant to the agent's purpose. For example:

- For a web search agent: "What is the capital of France?"
- For a news search agent: "Latest developments in artificial intelligence"
- For an image generator: "A sunset over a mountain lake with pine trees"

## Accessing Input in GenAIScript

In a GenAIScript file, the input is accessed through the `env.vars.user_input` property. It's common practice to assign this to a variable named "USER_INPUT" using the `def` function:

```typescript
def("USER_INPUT", env.vars.user_input);
```

You can then use this variable in your script's instructions:

```typescript
$`You are an assistant that performs a specific task.
- Use the USER_INPUT to guide your response
- ...other instructions...`
```

## Input Processing Examples

### Web Search Agent

```typescript
// In web-search.genai.mts
def("USER_INPUT", env.vars.user_input);

$`You are an assistant searching for web content using complex queries to pinpoint results.
- tailor search to answer the question in USER_INPUT
- ...other instructions...`
```

### News Search Agent

```typescript
// In news-search.genai.mts
def("USER_INPUT", env.vars.user_input);

$`You are an assistant searching for news using complex queries to pinpoint results.
- tailor search to answer the question in USER_INPUT
- ...other instructions...`
```

## Input Validation

Currently, there is no built-in validation for input. If your agent requires specific input formats or validation, you should implement this in your GenAIScript file. For example:

```typescript
def("USER_INPUT", env.vars.user_input);

// Simple validation example
if (!USER_INPUT || USER_INPUT.trim() === "") {
  $`Please provide a valid input query.`;
  exit();
}

// More complex validation could be implemented here
```

## Best Practices

1. **Be Clear About Input Requirements**: Document what kind of input your agent expects.
2. **Handle Edge Cases**: Consider how your agent will handle empty, very short, or very long inputs.
3. **Preprocess Input When Necessary**: If your agent needs input in a specific format, consider preprocessing it in the GenAIScript.
4. **Provide Examples**: Include example inputs in your agent's documentation.

## Testing Input

### Using the API

You can test how your agent handles different inputs using the API:

```bash
curl -X POST https://your-server.com/api/webhooks \
  -H "Authorization: Bearer <session_token>" \
  -H "Content-Type: application/json" \
  -d '{"resource": "your-resource-name", "input": "Your test input"}'
```

### Direct Invocation Examples

You can also run agents directly from the command line using the GenAIScript CLI or the shim. Here are examples from the project's package.json:

#### Web Search Agent

```bash
# Using genaiscript CLI
genaiscript run packages/genaiscript/genaisrc/web-search.genai.mts --vars USER_INPUT='who won the 2024 election?'

# Using the shim
./dist/shim.js --file=genaisrc/search.genai.mts USER_INPUT="Who won the 2024 presidential election?"
```

#### News Search Agent

```bash
genaiscript run packages/genaiscript/genaisrc/news-search.genai.mts --vars USER_INPUT='What are the latest updates and developments in the Ukraine war?'
```

#### Web Scrape Agent

```bash
# Read mode
genaiscript run packages/genaiscript/genaisrc/web-scrape.genai.mts --vars USER_INPUT='{"url":"https://geoff.seemueller.io/about","query":"Describe the details of the page.", "action": "read"}'

# Scrape mode
genaiscript run packages/genaiscript/genaisrc/web-scrape.genai.mts --vars USER_INPUT='{"url":"https://www.time4learning.com/homeschool-curriculum/high-school/eleventh-grade/math.html","query":"What is on this page?", "action": "scrape"}'
```

#### Finance Query Agent

```bash
# Crypto quote
genaiscript run packages/genaiscript/genaisrc/finance-query.genai.mts --vars USER_INPUT='Get a quote for BTC'

# Crypto news
genaiscript run packages/genaiscript/genaisrc/finance-query.genai.mts --vars USER_INPUT='What is the news for Bitcoin?'

# Market overview
genaiscript run packages/genaiscript/genaisrc/finance-query.genai.mts --vars USER_INPUT='What are the trending symbols in the market?'
```

Note the different input formats:
- Simple text queries for search and news agents
- JSON objects for the web scrape agent, specifying URL, query, and action
- Specific command-like queries for the finance agent

## Related Documentation

- [Agents Guide](./agents.md) - General information about agents
- [API Documentation](./api.md) - API endpoints and usage examples
