# universal-openai

A self-maintained, OpenAI-compatible API wrapper for Rust.

`universal-openai` is built for one idea: keep the wire format compatible with
the OpenAI API specification while making everyday Rust calls feel simple,
predictable, and provider-agnostic.

It is open source from the beginning and maintained under the GitHub identity
`edujbarrios` by Eduardo J. Barrios.

## Goals

- Simple calls for common LLM workflows.
- OpenAI-compatible request and response shapes.
- Works with OpenAI-compatible providers through a configurable base URL.
- Async-first HTTP client using `reqwest`.
- Small, readable API surface that is easy to maintain.

## Quick Start

```rust
use universal_openai::{Client, ChatMessage};

#[tokio::main]
async fn main() -> universal_openai::Result<()> {
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

## Provider-Agnostic Usage

```rust
use universal_openai::{Client, Config};

let client = Client::new(
    Config::new("your-api-key")
        .with_base_url("https://api.example.com/v1"),
)?;
```

Any service that follows the OpenAI-compatible `/chat/completions` format can
be called through the same client.

## Status

This project is intentionally small and early. The first public milestone is a
clean chat completions client, then streaming, embeddings, responses, and
provider-specific compatibility notes.

