# universal-openai-rs

[![CI](https://github.com/edujbarrios/universal-openai-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/edujbarrios/universal-openai-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

`universal-openai-rs` is a self-maintained, OpenAI-compatible API wrapper for
Rust with a strong focus on simple agent workflows.

Use it when you want:

- short calls for common AI tasks;
- native lightweight agents without a heavy framework;
- OpenAI-compatible request and response shapes when you need control;
- provider-agnostic clients for OpenAI, hosted compatible APIs, and local APIs.

## Install

This crate is not published to `crates.io` yet. Use it as a Git dependency or a
local path dependency.

Clone the repository:

```bash
git clone https://github.com/edujbarrios/universal-openai-rs.git
cd universal-openai-rs
```

Git dependency:

```toml
[dependencies]
universal-openai-rs = { git = "https://github.com/edujbarrios/universal-openai-rs.git" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Local path dependency:

```toml
[dependencies]
universal-openai-rs = { path = "../universal-openai-rs" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Minimal install for chat, responses, embeddings, models, images, moderations,
fine-tuning, agents, and typed tools:

```toml
[dependencies]
universal-openai-rs = {
    git = "https://github.com/edujbarrios/universal-openai-rs.git",
    default-features = false,
    features = ["rustls"]
}
```

Optional typed structured output:

```toml
[dependencies]
universal-openai-rs = {
    git = "https://github.com/edujbarrios/universal-openai-rs.git",
    features = ["structured-output"]
}
schemars = "0.8"
```

## Cargo Features

Default features are `rustls`, `stream`, and `multipart`.

| Feature | Enables |
| --- | --- |
| `rustls` | HTTPS with `rustls-tls` |
| `native-tls` | HTTPS with platform native TLS |
| `stream` | SSE/JSONL streaming helpers and stream dependencies |
| `multipart` | Multipart request support |
| `audio` | Audio transcription and translation builders |
| `files` | Files upload/list/retrieve/delete/download API |
| `agents` | Reserved marker for agent-focused builds |
| `structured-output` | Automatic JSON Schema from Rust structs via `schemars` |

Import it in Rust as:

```rust
use universal_openai_rs::prelude::*;
```

## Quick Start

```rust
use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let text = client.ask("gpt-4o-mini", "Write one sentence about Rust.").await?;

    println!("{text}");
    Ok(())
}
```

## Prompt-First API

Use `prompt(...)` for readable app code without manually building chat messages.

```rust
use universal_openai_rs::prelude::*;

let text = Client::from_env()?
    .prompt("Explain why Rust is useful for AI API clients.")
    .model("gpt-4o-mini")
    .system("Answer in one practical sentence.")
    .run_text()
    .await?;
```

## Responses-First API

Use `respond(...)` for Responses-style input, tools, structured output, and
typed parsing.

```rust
use serde::Deserialize;
use serde_json::json;
use universal_openai_rs::prelude::*;

#[derive(Deserialize)]
struct InvoiceSummary {
    status: String,
    amount: f64,
}

let summary: InvoiceSummary = client
    .respond("Summarize the latest invoice.")
    .model("gpt-4.1-mini")
    .json_schema_for::<InvoiceSummary>(json!({
        "type": "object",
        "properties": {
            "status": {"type": "string"},
            "amount": {"type": "number"}
        },
        "required": ["status", "amount"]
    }))
    .run_json()
    .await?;
```

With `structured-output`, generate schemas from Rust types:

```rust
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
struct Profile {
    title: String,
    strengths: Vec<String>,
}

let profile: Profile = client
    .respond("Return an AI engineer profile.")
    .model("gpt-4.1-mini")
    .run_structured()
    .await?;

let profile: Profile = client
    .ask_structured("gpt-4o-mini", "Return an AI engineer profile.")
    .await?;
```

## Native Lightweight Agents

Agents are intentionally lightweight: named instructions, task passing, optional
tools, and no external agent framework.

```rust
use universal_openai_rs::prelude::*;

let client = Client::from_env()?;

let agents = client
    .agents()
    .default_model("gpt-4o-mini")
    .simple("agent1", "Draft a concise technical answer.")
    .simple("agent2", "Review and improve the draft.");

let draft = agents
    .agent1("Design a simple OpenAI-compatible Rust call.")
    .await?;

let review_task = format!(
    "Use this draft as context, then improve it for a Rust developer:\n\n{}",
    draft.output
);

let reviewed = agents.agent2(review_task).await?;

println!("{}", reviewed.output);
```

For simple pipelines, `sequence(...)` passes each output to the next agent:

```rust
let run = agents
    .sequence(["agent1", "agent2"], "Design a simple OpenAI-compatible Rust call.")
    .await?;
```

Agents can carry executable tools. Arguments are decoded into typed Rust structs
before your function runs.

```rust
use serde::{Deserialize, Serialize};
use serde_json::json;
use universal_openai_rs::prelude::*;

#[derive(Deserialize)]
struct SearchArgs {
    query: String,
}

#[derive(Serialize)]
struct SearchOutput {
    answer: String,
}

let researcher = client.agent("researcher").tool_fn(
    "search_docs",
    "Search project documentation.",
    json!({
        "type": "object",
        "properties": {"query": {"type": "string"}},
        "required": ["query"]
    }),
    |args: SearchArgs| async move {
        Ok(SearchOutput {
            answer: format!("Found docs for {}", args.query),
        })
    },
);
```

For reusable tools, implement `AiTool` and register it through `ToolRegistry` or
`AgentSpec::ai_tool(...)`.

## Structured Builders

Use builders when you want OpenAI-compatible request shapes with typed Rust
helpers.

```rust
let response = client
    .chat()
    .model("gpt-4o-mini")
    .system("Be concise.")
    .user("Explain streaming.")
    .temperature(0.2)
    .send()
    .await?;
```

## Streaming

Requires the `stream` feature.

Use typed events for full deltas, or text chunks for UI/CLI rendering.

```rust
use futures_util::StreamExt;

let mut chunks = client
    .chat()
    .model("gpt-4o-mini")
    .user("Write a short Rust haiku.")
    .stream_text_chunks()
    .await?;

while let Some(chunk) = chunks.next().await {
    print!("{}", chunk?);
}
```

`stream_events()` returns typed `ChatStreamEvent` values. `stream()` is an alias.

For non-strict providers, pass another decoder:

```rust
let events = client
    .chat()
    .model("local-model")
    .user("Hello")
    .stream_events_with_decoder(universal_openai_rs::LenientSseDecoder::new())
    .await?;
```

## Provider-Agnostic Clients

```rust
use universal_openai_rs::{Client, Provider};

let openrouter = Client::for_provider("your-api-key", Provider::OpenRouter)?;
let ollama = Client::for_provider("ollama", Provider::Ollama)?;
let custom = Client::compatible("your-api-key", "https://api.example.com/v1")?;
```

## Production HTTP Configuration

Bring your own `reqwest::Client` for proxies, certificates, pooling, TCP
keepalive, or transport defaults.

```rust
use std::time::Duration;
use universal_openai_rs::{Client, Config};

let http = reqwest::Client::builder()
    .pool_idle_timeout(Duration::from_secs(90))
    .tcp_keepalive(Duration::from_secs(30))
    .build()?;

let config = Config::new("your-api-key")
    .with_base_url("https://api.example.com/v1")
    .with_user_agent("my-agent-service/0.1")
    .with_organization("org_123")
    .with_project("proj_123")
    .with_header("x-provider-routing", "fast");

let client = Client::with_http_client(config, http)?;
```

Retries support backoff, jitter, `Retry-After`, and retryable GET, DELETE,
multipart, JSON POST, timeout, and connection failures.

```rust
use std::time::Duration;
use universal_openai_rs::{Config, RetryConfig};

let config = Config::new("your-api-key").with_retry_config(RetryConfig {
    max_retries: 5,
    initial_backoff: Duration::from_millis(250),
    max_backoff: Duration::from_secs(20),
    jitter: true,
    respect_retry_after: true,
});
```

## Adding an OpenAI-Compatible Third-Party API

Set a compatible `/v1` base URL. Example with `llm7.io`:

```rust
use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::compatible("your-api-key", "https://api.llm7.io/v1")?;

    let text = client
        .ask("gpt-4o-mini", "Explain OpenAI-compatible APIs in one sentence.")
        .await?;

    println!("{text}");
    Ok(())
}
```

Or use environment variables:

```bash
OPENAI_API_KEY=your-api-key
OPENAI_BASE_URL=https://api.llm7.io/v1
OPENAI_MODEL=gpt-4o-mini
```

## Common Workflows

```rust
let text = client.ask("gpt-4o-mini", "Hello").await?;
let json: serde_json::Value = client.ask_json("gpt-4o-mini", "Return JSON").await?;
let vector = client.embed("text-embedding-3-small", "Embed this").await?;
let image = client.generate_image("gpt-image-1", "A clean Rust API diagram").await?;
let moderation = client.moderate_text("Text to classify").await?;
```

Vision inputs support image URLs and base64 data URLs such as:

```text
data:image/png;base64,...
```

With the `files` feature, files can be uploaded, listed, inspected, deleted, and
downloaded with `client.files().content(file_id)`.

## Endpoint Coverage

| API area | Support |
| --- | --- |
| Models | `client.models()` / `client.list_models()` |
| Completions | `client.completions()` / `client.complete_text(...)` |
| Chat | `client.chat()` / `client.prompt(...)` |
| Responses | `client.respond(...)` / `client.responses()` / `client.respond_text(...)` |
| Embeddings | `client.embeddings()` / `client.embed(...)` |
| Images | `client.images()` / `client.generate_image(...)` |
| Audio | `client.audio()` / `client.transcribe(...)` with `audio` |
| Files | `client.files()` / `client.upload_file(...)` with `files` |
| Fine-tuning | `client.fine_tuning()` |
| Moderations | `client.moderations()` / `client.moderate_text(...)` |
| Agents | `client.agents()` |
| Tool execution | `ToolRegistry` / `AgentSpec::tool_fn(...)` |
| Engines | Legacy only via compatibility escape hatches |

## Escape Hatches

If a provider supports an endpoint or option before this crate exposes a typed
builder, use the compatibility methods.

```rust
use serde_json::{json, Value};

let response: Value = client
    .send_compatible(
        "chat/completions",
        &json!({
            "model": "gpt-4o-mini",
            "messages": [{"role": "user", "content": "Hello"}]
        }),
    )
    .await?;
```

Available escape hatches:

- `send_compatible(...)`
- `get_compatible(...)`
- `delete_compatible(...)`
- `.extra(...)` on builders for provider-specific fields

## API Errors

Provider errors keep the raw body plus structured fields when available.

```rust
match error {
    universal_openai_rs::Error::Api(api) => {
        eprintln!("status: {}", api.status);
        eprintln!("request id: {:?}", api.request_id);
        eprintln!("provider code: {:?}", api.code);
    }
    other => eprintln!("{other}"),
}
```

## Current Caveats

This project is early. Known issue areas include provider-specific response
shapes, non-standard streaming, large multipart uploads, and legacy `Engines`.

See [docs/known-issues.md](docs/known-issues.md).

## Checks

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --no-default-features --features rustls
cargo check --no-default-features --features native-tls
cargo check --all-targets --features "audio files structured-output stream"
cargo check --examples
cargo test
cargo test --features "audio files structured-output stream"
cargo doc --no-deps
```

## Project Docs

- [Design philosophy](docs/design.md)
- [Provider compatibility](docs/providers.md)
- [Known issues](docs/known-issues.md)
- [Implemented coverage](docs/status.md)
- [Examples](examples/README.md)
