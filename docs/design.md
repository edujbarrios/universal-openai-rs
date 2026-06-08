# Design Philosophy

`universal-openai-rs` is designed around a simple principle:

Make OpenAI-compatible API calls easy and well structured in Rust without hiding
the underlying API shape.

## Simple by Default

The shortest useful call should be short:

```rust
let text = client.ask("gpt-4o-mini", "Explain Rust ownership.").await?;
```

For common application code, developers should not need to construct request
JSON manually or remember every endpoint path.

## Structured When Needed

When a workflow needs more control, the SDK exposes builders that map closely to
OpenAI-compatible request bodies:

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

This keeps code readable while preserving the provider-compatible JSON shape.

## Prompt-First API

The prompt-first API is the main differentiator. It starts with the developer's
intent instead of an endpoint:

```rust
let text = client
    .prompt("Explain why Rust is useful for AI API clients.")
    .model("gpt-4o-mini")
    .system("Answer in one practical sentence.")
    .run_text()
    .await?;
```

That workflow remains compatible with tools, structured output, streaming, and
default models.

## Provider Compatibility

OpenAI-compatible providers often agree on the broad API shape but differ in
small options, beta fields, and rollout timing. This crate handles that with
three layers:

- First-class typed builders for common workflows.
- `extra(...)` fields for provider-specific JSON options.
- `send_compatible(...)` for new or unusual endpoints.

The goal is to avoid provider lock-in while keeping normal calls clean.

## Maintenance Goals

- Keep public APIs small and composable.
- Prefer typed request/response shapes where they clarify code.
- Keep raw JSON access available where providers move faster than SDKs.
- Add examples for real workflows, not only isolated endpoints.
- Keep the crate understandable to contributors.

