# universal-openai-rs

[![CI](https://github.com/edujbarrios/universal-openai-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/edujbarrios/universal-openai-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

`universal-openai-rs` is a self-maintained, OpenAI-compatible APIs wrapper for
Rust. Optimized for Agents.

The main purpose is to make AI API calls in Rust simple to write, simple to
read, and especially simple to compose into lightweight agent workflows, while
staying close to the OpenAI-compatible request/response format when you need
full control.

## About

A self-maintained, OpenAI-compatible APIs wrapper for Rust. Optimized for
Agents.

This wrapper puts a huge focus on simplicity while working with agents: define
small named agents, pass tasks between them, and keep the underlying API calls
provider-compatible.

## What This Project Is

This is an AI engineering utility crate for:

- building agent-style workflows without pulling in a heavy agent framework;
- calling OpenAI-compatible providers from Rust;
- keeping common workflows short with helpers like `ask`, `prompt`, and `embed`;
- using structured builders for chat, responses, embeddings, images, audio,
  files, models, fine-tuning, moderations, and completions;
- keeping escape hatches for provider-specific or newly released endpoints.

It is designed for local development, experimentation, and open source evolution,
with agent simplicity as a primary design goal.

## Install Status

This crate is not published to `crates.io` yet. It is not a Python package and
is not available on PyPI.

For now, use it as a local crate or as a Git dependency.

## Clone

```bash
git clone https://github.com/edujbarrios/universal-openai-rs.git
cd universal-openai-rs
```

## Use From Another Rust Project

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

Use `prompt(...)` when you want readable application code without manually
constructing chat messages.

```rust
use universal_openai_rs::prelude::*;

let text = Client::from_env()?
    .prompt("Explain why Rust is useful for AI API clients.")
    .model("gpt-4o-mini")
    .system("Answer in one practical sentence.")
    .run_text()
    .await?;
```

## Native Lightweight Agents

Agent workflows are a first-class focus of the crate. The native agent layer is
intentionally lightweight: no scheduler, database, or external agent framework.

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

For simple pipelines, `sequence(...)` runs agents in order and passes each output
to the next agent:

```rust
let run = agents
    .sequence(["agent1", "agent2"], "Design a simple OpenAI-compatible Rust call.")
    .await?;
```

## Structured Builders

Use builders when you want a request shape that stays close to the
OpenAI-compatible API.

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

## Provider-Agnostic Clients

```rust
use universal_openai_rs::{Client, Provider};

let openrouter = Client::for_provider("your-api-key", Provider::OpenRouter)?;
let ollama = Client::for_provider("ollama", Provider::Ollama)?;
let custom = Client::compatible("your-api-key", "https://api.example.com/v1")?;
```

## Production HTTP Configuration

For production clients, keep the simple crate API while bringing your own
`reqwest::Client` for proxies, certificates, connection pooling, TCP keepalive,
global defaults, or custom transport settings.

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

## Adding an OpenAI-Compatible Third-Party API

Any provider that exposes an OpenAI-compatible `/v1` API can be used by changing
the base URL.

Example with `llm7.io`:

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

You can also use environment variables:

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

Files can be uploaded, listed, inspected, deleted, and downloaded with
`client.files().content(file_id)`.

## Endpoint Coverage

| API area | Support |
| --- | --- |
| Models | `client.models()` / `client.list_models()` |
| Completions | `client.completions()` / `client.complete_text(...)` |
| Chat | `client.chat()` / `client.prompt(...)` |
| Responses | `client.responses()` / `client.respond_text(...)` |
| Embeddings | `client.embeddings()` / `client.embed(...)` |
| Images | `client.images()` / `client.generate_image(...)` |
| Audio | `client.audio()` / `client.transcribe(...)` |
| Files | `client.files()` / `client.upload_file(...)` |
| Fine-tuning | `client.fine_tuning()` |
| Moderations | `client.moderations()` / `client.moderate_text(...)` |
| Agents | `client.agents()` |
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

Provider errors keep the raw response body and also expose structured debugging
fields when the provider returns OpenAI-compatible error JSON.

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
shapes, non-standard streaming formats, large multipart uploads, and legacy
`Engines` compatibility.

See [docs/known-issues.md](docs/known-issues.md).

## Checks

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --examples
cargo test
cargo doc --no-deps
```

## Project Docs

- [Design philosophy](docs/design.md)
- [Provider compatibility](docs/providers.md)
- [Known issues](docs/known-issues.md)
- [Implemented coverage](docs/status.md)
- [Examples](examples/README.md)
