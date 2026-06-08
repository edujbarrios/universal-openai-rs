# universal-openai-rs

[![CI](https://github.com/edujbarrios/universal-openai-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/edujbarrios/universal-openai-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A self-maintained, OpenAI-compatible API wrapper for Rust.

Simple by default. Structured when needed. Compatible by design.

`universal-openai-rs` makes easy, well-structured API calls to OpenAI-compatible
providers while keeping the underlying API shape available.

## Why It Stands Out

- Intent-first helpers: `ask`, `prompt`, `embed`, `respond_text`.
- Native lightweight agents: `agents().agent1(task)` and `agents().sequence(...)`.
- OpenAI-compatible builders for chat, responses, embeddings, images, audio,
  files, models, fine-tuning, moderations, and legacy completions.
- Provider presets for OpenAI, OpenRouter, Groq, Together, Ollama, and custom
  OpenAI-compatible base URLs.
- Escape hatches for new or provider-specific endpoints:
  `send_compatible`, `get_compatible`, and `delete_compatible`.

## Clone

```bash
git clone https://github.com/edujbarrios/universal-openai-rs.git
cd universal-openai-rs
```

Repository:
[github.com/edujbarrios/universal-openai-rs](https://github.com/edujbarrios/universal-openai-rs/tree/main)

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

`Client::from_env()` reads:

- `OPENAI_API_KEY`
- `OPENAI_BASE_URL`, optional
- `OPENAI_MODEL`, optional

## Prompt-First API

```rust
use universal_openai_rs::prelude::*;

let text = Client::from_env()?
    .prompt("Explain why Rust is useful for AI API clients.")
    .model("gpt-4o-mini")
    .system("Answer in one practical sentence.")
    .run_text()
    .await?;
```

## Native Agents

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

For simple pipelines, `sequence(...)` runs agents in order and passes each
output to the next agent:

```rust
let run = agents
    .sequence(["agent1", "agent2"], "Design a simple OpenAI-compatible Rust call.")
    .await?;
```

## Provider-Agnostic Usage

```rust
use universal_openai_rs::{Client, Provider};

let openrouter = Client::for_provider("your-api-key", Provider::OpenRouter)?;
let ollama = Client::for_provider("ollama", Provider::Ollama)?;
let custom = Client::compatible("your-api-key", "https://api.example.com/v1")?;
```

## Structured Builders

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

## Common Workflows

```rust
let text = client.ask("gpt-4o-mini", "Hello").await?;
let json: MyType = client.ask_json("gpt-4o-mini", "Return JSON").await?;
let vector = client.embed("text-embedding-3-small", "Embed this").await?;
let image = client.generate_image("gpt-image-1", "A clean Rust API diagram").await?;
let moderation = client.moderate_text("Text to classify").await?;
```

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

Vision inputs accept image URLs or base64 data URLs such as
`data:image/png;base64,...`.

## Escape Hatch

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

## Checks

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps
```

## More

- [Design philosophy](docs/design.md)
- [Provider compatibility](docs/providers.md)
- [Implemented coverage](docs/status.md)
- [Examples](examples/README.md)
