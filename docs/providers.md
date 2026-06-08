# Provider Compatibility

`universal-openai-rs` targets OpenAI-compatible APIs rather than a single
provider.

The crate includes presets for common providers:

| Provider | Base URL |
| --- | --- |
| `Provider::OpenAI` | `https://api.openai.com/v1` |
| `Provider::OpenRouter` | `https://openrouter.ai/api/v1` |
| `Provider::Groq` | `https://api.groq.com/openai/v1` |
| `Provider::Together` | `https://api.together.xyz/v1` |
| `Provider::Ollama` | `http://localhost:11434/v1` |
| `Provider::Custom(...)` | Any OpenAI-compatible base URL |

## Preset Usage

```rust
use universal_openai_rs::{Client, Provider};

let client = Client::for_provider("your-api-key", Provider::OpenRouter)?;
let text = client.ask("openai/gpt-4o-mini", "Say hello.").await?;
```

## Local Model Usage

```rust
use universal_openai_rs::{Client, Provider};

let client = Client::for_provider("ollama", Provider::Ollama)?;
let text = client.ask("llama3.2", "Say hello from a local model.").await?;
```

## Custom Provider Usage

```rust
use universal_openai_rs::Client;

let client = Client::compatible("your-api-key", "https://api.example.com/v1")?;
```

## Handling Provider Differences

OpenAI-compatible providers may support extra parameters. Use `extra(...)` when
the endpoint is already typed:

```rust
let response = client
    .chat()
    .model("provider/model")
    .user("Hello")
    .extra("provider_specific_option", true)
    .send()
    .await?;
```

Use `send_compatible(...)` when the endpoint itself is not typed yet.

