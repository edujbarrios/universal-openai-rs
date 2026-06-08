# universal-openai-rs

[![CI](https://github.com/edujbarrios/universal-openai-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/edujbarrios/universal-openai-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A self-maintained, OpenAI-compatible API wrapper for Rust.

Simple by default. Structured when needed. Compatible by design.

`universal-openai-rs` is built for one idea: keep the wire format compatible with
the OpenAI API specification while making everyday Rust calls feel simple,
predictable, and provider-agnostic.

The intention of this repo is to make easy, well-structured API calls for
OpenAI-compatible APIs. The design is simple by default and spec-compatible when
needed.

Most Rust SDKs start from the endpoint. `universal-openai-rs` starts from the
developer's intent:

- `ask(...)` for one-line text generation.
- `prompt(...)` for a readable prompt-first workflow.
- `chat()` when you want full OpenAI-compatible chat completions.
- `responses()` and `embeddings()` for newer API surfaces.
- `send_compatible(...)` when a provider adds a feature before the crate does.

It is open source from the beginning and maintained under the GitHub identity
`edujbarrios` by Eduardo J. Barrios.

## Goals

- Simple calls for common LLM workflows.
- OpenAI-compatible request and response shapes.
- Works with OpenAI-compatible providers through a configurable base URL.
- Chat completions, streaming, embeddings, and Responses API support.
- Structured output and tool calling without provider lock-in.
- Configurable timeouts and retries for production-friendly usage.
- Async-first HTTP client using `reqwest`.
- Small, readable API surface that is easy to maintain.

## What Makes It Different

`universal-openai-rs` is not trying to hide the OpenAI-compatible spec. It keeps
that shape available, but wraps it with a Rust-friendly experience:

- Intent-first calls: `ask`, `ask_json`, `prompt`, `embed`, and `respond_text`.
- Spec builders: `chat`, `responses`, and `embeddings` map cleanly to provider
  JSON.
- Provider presets for common OpenAI-compatible APIs.
- A raw compatibility escape hatch for endpoints and provider options that are
  not typed yet.
- Typed structured output helpers without forcing a framework.
- Small public types that are easy to inspect, serialize, test, and extend.

## API Surface

| Need | Simple API | Structured API |
| --- | --- | --- |
| Text generation | `client.ask(...)` | `client.chat().send()` |
| Prompt workflow | `client.prompt(...).run_text()` | `client.prompt(...).into_chat()` |
| Typed JSON | `client.ask_json::<T>(...)` | `.json_schema(...).send()` |
| Streaming | `.stream_text()` | `.stream()` |
| Embeddings | `client.embed(...)` | `client.embeddings().send()` |
| Responses API | `client.respond_text(...)` | `client.responses().send()` |
| Provider-specific fields | `.extra(...)` | `.send_compatible(...)` |

## Quick Start

For the shortest common path:

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

For a more expressive prompt-first workflow:

```rust
use universal_openai_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::from_env()?;
    let text = client
        .prompt("Explain why Rust is useful for AI API clients.")
        .model("gpt-4o-mini")
        .system("Answer in one practical sentence.")
        .run_text()
        .await?;

    println!("{text}");
    Ok(())
}
```

For a full OpenAI-compatible chat request:

```rust
use universal_openai_rs::{Client, ChatMessage};

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;

    let response = client
        .chat()
        .model("gpt-4o-mini")
        .message(ChatMessage::user("Write one sentence about Rust."))
        .send()
        .await?;

    println!("{}", response.first_text().unwrap_or_default());
    Ok(())
}
```

By default, `Client::from_env()` reads:

- `OPENAI_API_KEY`
- `OPENAI_BASE_URL`, optional, defaults to `https://api.openai.com/v1`
- `OPENAI_MODEL`, optional, used by `ask_default(...)` and `chat_default()`

With `OPENAI_MODEL` set, the shortest call becomes:

```rust
let text = client.ask_default("Write one sentence about Rust.").await?;
```

You can also import the common surface with:

```rust
use universal_openai_rs::prelude::*;
```

## Provider-Agnostic Usage

```rust
use universal_openai_rs::{Client, Config, Provider};

let openrouter = Client::for_provider("your-api-key", Provider::OpenRouter)?;
let local = Client::for_provider("ollama", Provider::Ollama)?;
let custom = Client::compatible("your-api-key", "https://api.example.com/v1")?;
```

Any service that follows the OpenAI-compatible `/chat/completions` format can
be called through the same client.

You can still build a client manually when you want more control:

```rust
let client = Client::new(
    Config::new("your-api-key")
        .with_base_url("https://api.example.com/v1"),
)?;
```

## Streaming

```rust
use futures_util::StreamExt;
use universal_openai_rs::Client;

let client = Client::from_env()?;
let mut stream = client
    .chat()
    .model("gpt-4o-mini")
    .user("Write a short Rust haiku.")
    .stream()
    .await?;

while let Some(event) = stream.next().await {
    for choice in event?.choices {
        if let Some(text) = choice.delta.content {
            print!("{text}");
        }
    }
}
```

## Embeddings

```rust
let vector = client
    .embed("text-embedding-3-small", "Rust makes API clients reliable.")
    .await?;
```

## Responses API

```rust
let response = client
    .respond_text("gpt-4o-mini", "Explain provider-agnostic APIs in one sentence.")
    .await?;

println!("{}", response.output_text.unwrap_or_default());
```

## Structured Output

```rust
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct EngineerProfile {
    title: String,
    strengths: Vec<String>,
}

let profile: EngineerProfile = client
    .ask_json("gpt-4o-mini", "Return a compact profile for an AI engineer.")
    .await?;
```

Prompt-first structured output is available too:

```rust
let profile: EngineerProfile = client
    .prompt("Return a compact profile for an AI engineer.")
    .model("gpt-4o-mini")
    .run_json()
    .await?;
```

When you want to pass an explicit JSON schema:

```rust
let response = client
    .chat()
    .model("gpt-4o-mini")
    .user("Return a compact profile for an AI engineer.")
    .json_schema(
        "engineer_profile",
        json!({
            "type": "object",
            "properties": {
                "title": {"type": "string"},
                "strengths": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            },
            "required": ["title", "strengths"]
        }),
    )
    .send()
    .await?;
```

## OpenAI-Compatible Escape Hatch

If a provider supports a new endpoint before this crate adds first-class types,
send the OpenAI-compatible JSON yourself:

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

## Tool Calling

```rust
use serde_json::json;
use universal_openai_rs::Tool;

let response = client
    .chat()
    .model("gpt-4o-mini")
    .user("What should I pack for Madrid today?")
    .tool(Tool::function(
        "get_weather",
        "Get weather for a city.",
        json!({
            "type": "object",
            "properties": {
                "city": {"type": "string"}
            },
            "required": ["city"]
        }),
    ))
    .send()
    .await?;
```

## Timeouts and Retries

```rust
use std::time::Duration;
use universal_openai_rs::{Client, Config};

let client = Client::new(
    Config::new("your-api-key")
        .with_timeout(Duration::from_secs(30))
        .with_max_retries(3),
)?;
```

## Status

This project is intentionally small and early, but the first useful API surface
now covers chat completions, streaming, embeddings, Responses API, structured
output, tool calling, retries, timeouts, and provider-specific extension fields.

See [ROADMAP.md](ROADMAP.md) for the public development path.

See [docs/design.md](docs/design.md) for the design philosophy and
[docs/providers.md](docs/providers.md) for provider compatibility notes.
