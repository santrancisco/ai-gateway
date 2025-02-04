<div align="center">

# LangDB AI Gateway

A Rust-based gateway service for interacting with various LLM providers (OpenAI, Anthropic, etc.) with unified API interface.

<img src="assets/langdb-models.gif" width="550px" alt="LangDB AI Gateway Demo showing LLM Switching">
</div>

## Table of Contents
- [Features](#features)
- [Setup](#setup)
  - [Quick Start](#quick-start)
  - [Using Docker](#using-docker)
  - [Direct Installation](#direct-installation)
  - [Build from Source](#build-from-source)
  - [Running with Options](#running-with-options)
    - [Command Line Options](#command-line-options)
    - [Using Config File](#using-config-file)
    - [Rate Limiting](#rate-limiting)
- [API Endpoints](#api-endpoints)
- [Supported Providers](#supported-providers)
- [Tracing](#tracing)
  - [Setting up Tracing](#setting-up-tracing)
  - [Querying Traces](#querying-traces)


## Features

- OpenAI-compatible API endpoints
- Model configuration via YAML
- Support for multiple LLM providers
- Debug-level event logging
- OpenTelemetry integration
- Cost tracking and usage monitoring

## Setup

### Quick Start

Choose one of the following scenarios to get started:

#### Configuration

You can configure the API keys and other settings in two ways:

##### 1. Using config.yaml

Create a `config.yaml` file (or copy from `config.sample.yaml`):
```yaml
http:
  host: "0.0.0.0"
  port: 8080

providers:
  openai: 
    api_key: "your-openai-key-here"
  anthropic: 
    api_key: "your-anthropic-key-here"
  # Add other providers as needed
```

##### 2. Using Environment Variables

Alternatively, create a `.env` file:
```bash
# API Keys for different providers
LANGDB_OPENAI_API_KEY=your-openai-key-here
LANGDB_ANTHROPIC_API_KEY=your-anthropic-key-here
LANGDB_GEMINI_API_KEY=your-gemini-key-here
LANGDB_BEDROCK_API_KEY=your-bedrock-key-here
LANGDB_DEEPSEEK_API_KEY=your-deepseek-key-here
LANGDB_TOGETHERAI_API_KEY=your-togetherai-key-here
LANGDB_XAI_API_KEY=your-xai-key-here

# Optional: Set log level (default: info)
RUST_LOG=debug
```

The service will automatically load configuration from both sources, with environment variables taking precedence over config file values.

### Using Docker

```bash
# Pull and run the container
docker run -it \
    -p 8080:8080 \
    -e LANGDB_OPENAI_API_KEY=$OPENAI_API_KEY \
    -e RUST_LOG=info \
    -v $(pwd)/config.yaml:/app/config.yaml \
    langdb/ai-gateway serve
```
Available commands:
```bash
docker run -it langdb/ai-gateway 
```

#### Make your first request

```bash
curl http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-4o-mini",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```


### Direct installation

1. Install using cargo:
```bash
RUSTFLAGS="--cfg tracing_unstable --cfg aws_sdk_unstable" cargo install --git https://github.com/langdb/ai-gateway.git
```

2. Run the server:
```bash
ai-gateway serve
```

### Build from source

1. Clone the repository:
```bash
git clone https://github.com/langdb/ai-gateway.git
cd ai-gateway
```

2. Run the server with default configuration:
```bash
cargo run -- serve
```

> Both scenarios will start the server on `0.0.0.0:8080` with default settings.



### Running with Options

The server supports various configuration options that can be specified either via command line arguments or through a config file.

#### Command Line Options

```bash
# Run with custom host and port
cargo run -- serve --host 0.0.0.0 --port 3000

# Run with CORS origins
cargo run -- serve --cors-origins "http://localhost:3000,http://example.com"

# Run with rate limiting
cargo run -- serve --rate-hourly 1000

# Run with cost limits
cargo run -- serve --cost-daily 100.0 --cost-monthly 1000.0

# Run with custom database connections
cargo run -- serve --clickhouse-url "clickhouse://localhost:9000"
```

#### Using Config File

1. Copy the example config file:
```bash
cp config.sample.yaml config.yaml
```

2. Run the server:
```bash
cargo run -- serve
```

Command line options will override corresponding config file settings when both are specified.

### Rate Limiting

Rate limiting helps prevent API abuse by limiting the number of requests within a time window. Configure rate limits using:

```bash
# Limit to 1000 requests per hour
cargo run -- serve --rate-hourly 1000
```

Or in `config.yaml`:
```yaml
rate_limit:
  hourly: 1000
```

When a rate limit is exceeded, the API will return a 429 (Too Many Requests) response.

## API Endpoints

The gateway provides the following OpenAI-compatible endpoints:

- `POST /v1/chat/completions` - Chat completions
- `GET /v1/models` - List available models
- `POST /v1/embeddings` - Generate embeddings
- `POST /v1/images/generations` - Generate images

## Supported Providers

LangDB AI Gateway currently supports the following LLM providers:

|  | Provider |
|------|----------|
| <img src="assets/images/openai.png" width="32"> | OpenAI |
| <img src="assets/images/gemini.png" width="32"> | Google Gemini |
| <img src="assets/images/Anthropic-AI.png" width="32"> | Anthropic |
| <img src="assets/images/deepseek.png" width="32"> | DeepSeek |
| <img src="assets/images/cohere.875858bb.svg" width="32"> | TogetherAI |
| <img src="assets/images/xai.png" width="32"> | XAI |
| <img src="assets/images/meta.png" width="32"> | Meta ( Provided by Bedrock ) |
| <img src="assets/images/cohere.png" width="32"> | Cohere ( Provided by Bedrock ) |
| <img src="assets/images/mistral.png" width="32"> | Mistral ( Provided by Bedrock ) |

## Tracing

The gateway supports OpenTelemetry tracing with ClickHouse as the storage backend. All traces are stored in the `langdb.traces` table.

### Setting up Tracing

1. Create the traces table in ClickHouse:
```bash
# Create langdb database if it doesn't exist
clickhouse-client --query "CREATE DATABASE IF NOT EXISTS langdb"

# Import the traces table schema
clickhouse-client --query "$(cat sql/traces.sql)"
```

2. Enable tracing by providing the ClickHouse URL when running the server:
```bash
cargo run -- serve --clickhouse-url "clickhouse://localhost:9000"
```

You can also set the URL in your `config.yaml`:
```yaml
clickhouse:
  url: "http://localhost:8123"
```

### Querying Traces

The traces are stored in the `langdb.traces` table. Here are some example queries:

```sql
-- Get recent traces
SELECT 
    trace_id,
    operation_name,
    start_time_us,
    finish_time_us,
    (finish_time_us - start_time_us) as duration_us
FROM langdb.traces
WHERE finish_date >= today() - 1
ORDER BY finish_time_us DESC
LIMIT 10;
```
### Cost Control

Cost control helps manage API spending by setting daily, monthly, or total cost limits. Configure cost limits using:

```bash
# Set daily and monthly limits
cargo run -- serve \
  --cost-daily 100.0 \
  --cost-monthly 1000.0 \
  --cost-total 5000.0
```

Or in `config.yaml`:
```yaml
cost_control:
  daily: 100.0   # $100 per day
  monthly: 1000.0  # $1000 per month
  total: 5000.0    # $5000 total
```

When a cost limit is reached, the API will return a 429 response with a message indicating the limit has been exceeded.

## Running with Docker Compose

For a complete setup including ClickHouse for analytics and tracing, follow these steps:

1. Start the services using Docker Compose:
```bash
docker-compose up -d
```

This will start:
- ClickHouse server on ports 8123 (HTTP)
- All necessary configurations will be loaded from `docker/clickhouse/server/config.d`

2. Build and run the gateway:
```bash
cargo run
```

The gateway will now be running with full analytics and logging capabilities, storing data in ClickHouse.

## Using MCP Tools
```bash
curl http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-4o-mini",
    "messages": [{"role": "user", "content": "Ping the server using the tool and return the response"}],
    "mcp_servers": [{"server_url": "http://localhost:3004"}]
  }'
```

## Development

### Project Structure

- `gateway/` - Core gateway implementation
  - Models and provider integrations
  - API types and handlers
  - OpenTelemetry integration
- `server/` - HTTP server implementation
  - Configuration management
  - REST API endpoints
  - Cost tracking

### Running Tests

```bash
cargo test
```

### Logging

The gateway uses `tracing` for logging. Set the `RUST_LOG` environment variable to control log levels:

```bash
RUST_LOG=debug cargo run    # For detailed logs
RUST_LOG=info cargo run     # For standard logs
```

## License

This project is released under the [Apache License 2.0](./LICENSE.md). See the license file for more information.


## Roadmap

- [x] Include License (Apache2)
- [x] clickhouse config + traces
- [x] Provide example docker-compose (simple / full (clickhouse))
- [x] cost control
- [x] rate limiting
- [ ] cargo install / curl -sH install
- [ ] CI/CD for ubuntu / mac silicon
- [ ] postman 
- [ ] Include Model selection config (All / Filter)
- [ ] usage command (runs a query and prints model usage)
- [ ] README has explanations each of them.
- [ ] Docs (opensource section) / Mrunmay
