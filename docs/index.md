# web-agent-rs Documentation

Welcome to the documentation for web-agent-rs, a GenAIScript host for integration into conversational AI applications.

## Table of Contents

- [Installation Guide](./installation.md) - How to install and set up the project
- [Configuration Guide](./configuration.md) - Environment variables and configuration options
- [API Documentation](./api.md) - API endpoints and usage examples
- [Authentication](./tokens.md) - Authentication system documentation
- [Agents Guide](./agents.md) - How to create and use agents

## Overview

web-agent-rs is a server that hosts GenAIScript agents for integration into conversational AI applications. It provides a simple API for creating and consuming stream resources that execute various agents to perform tasks like web search, news search, image generation, finance queries, and web scraping.

## Architecture

The application is built with Rust using the Axum web framework. It uses GenAIScript for defining agent behavior and provides a streaming API for consuming agent responses.

### Key Components

1. **Server** - The main application server that handles HTTP requests and responses
2. **Agents** - GenAIScript files that define agent behavior
3. **Handlers** - Rust functions that process HTTP requests
4. **Authentication** - FIPS204 signature-based authentication system
5. **Configuration** - Environment variables for configuring the application

## Getting Started

To get started with web-agent-rs, follow these steps:

1. Read the [Installation Guide](./installation.md) to set up the project
2. Configure the application using the [Configuration Guide](./configuration.md)
3. Learn how to use the API with the [API Documentation](./api.md)
4. Understand the authentication system with the [Authentication](./tokens.md) documentation
5. Create your own agents using the [Agents Guide](./agents.md)

## Security Considerations

Please note that this project has not undergone a formal security assessment. You should do your own evaluation before using it in production environments.

## Contributing

Contributions to web-agent-rs are welcome! Please feel free to submit issues and pull requests to improve the project.