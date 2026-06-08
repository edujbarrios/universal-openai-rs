# Known Issues and Troubleshooting

This project is early and intentionally provider-agnostic. Most issues will come
from provider differences, authentication, or unsupported edge cases.

## Cargo Is Not Installed

If `cargo test` or `cargo run --example simple` fails with `cargo` not found,
install Rust from <https://rustup.rs/> and reopen the terminal.

## Missing API Key

`Client::from_env()` requires `OPENAI_API_KEY`.

```bash
OPENAI_API_KEY=your-api-key
```

For compatible providers, also set:

```bash
OPENAI_BASE_URL=https://api.example.com/v1
OPENAI_MODEL=provider-model-name
```

## Wrong Base URL

Use the provider's OpenAI-compatible `/v1` base URL. Do not include endpoint
paths such as `/chat/completions` in `OPENAI_BASE_URL`.

Correct:

```bash
OPENAI_BASE_URL=https://api.openai.com/v1
```

Incorrect:

```bash
OPENAI_BASE_URL=https://api.openai.com/v1/chat/completions
```

## Provider-Specific Fields

If a provider needs a field this crate does not expose yet, use `extra(...)`:

```rust
let response = client
    .chat()
    .model("provider/model")
    .user("Hello")
    .extra("provider_option", true)
    .send()
    .await?;
```

If the endpoint itself is missing, use `send_compatible(...)`,
`get_compatible(...)`, or `delete_compatible(...)`.

## Streaming Differences

Streaming assumes Server-Sent Events with `data:` lines and `[DONE]` as the end
marker. Some local or proxy providers may format streaming differently. In that
case, use non-streaming `.send()` or open an issue with the provider response
shape.

## Multipart Audio and File Uploads

Audio and file APIs use in-memory bytes. Large files may need a future streaming
upload API. For now, prefer reasonably sized inputs.

## Responses API Text Extraction

`ResponsesResponse::text()` first uses `output_text`, then falls back to
`output[].content[].text`. If a provider returns a different shape, inspect
`ResponsesResponse.output` or `ResponsesResponse.extra`.

## Engines

The legacy `Engines` API is not first-class. Use compatibility escape hatches if
a provider still exposes it.

