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

## Native Agent Layer

The agent layer is intentionally lightweight. It does not require a separate
runtime, database, scheduler, or framework. An agent is a named prompt policy
that runs through the same universal OpenAI-compatible client:

```rust
let agents = client
    .agents()
    .default_model("gpt-4o-mini")
    .simple("agent1", "Answer as a concise Rust AI engineer.")
    .simple("agent2", "Review the answer and suggest one improvement.");

let first = agents.agent1("Design a simple API call.").await?;
let second = agents.agent2(first.output).await?;
```

For chained work, `sequence(...)` runs agents in order and passes each output to
the next agent:

```rust
let run = agents
    .sequence(["agent1", "agent2"], "Design a simple API call.")
    .await?;
```

This gives the crate native agent ergonomics while keeping the underlying API
simple and inspectable.

## Provider Compatibility

OpenAI-compatible providers often agree on the broad API shape but differ in
small options, beta fields, and rollout timing. This crate handles that with
three layers:

- First-class typed builders for common workflows.
- `extra(...)` fields for provider-specific JSON options.
- `send_compatible(...)`, `get_compatible(...)`, and `delete_compatible(...)`
  for new or unusual endpoints.

The goal is to avoid provider lock-in while keeping normal calls clean.

## Maintenance Goals

- Keep public APIs small and composable.
- Prefer typed request/response shapes where they clarify code.
- Keep raw JSON access available where providers move faster than SDKs.
- Add examples for real workflows, not only isolated endpoints.
- Keep the crate understandable to contributors.
