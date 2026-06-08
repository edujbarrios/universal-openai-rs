# Roadmap

`universal-openai-rs` starts with chat completions and grows outward without making
the simple path harder to use.

## Milestone 1: Chat Completions

- Builder API for `/chat/completions`.
- One-call text helper for simple prompts.
- OpenAI-compatible JSON request and response structures.
- Extra JSON fields for provider-specific options.
- Structured output helpers.
- Tool calling helpers.

## Milestone 2: Streaming

- Server-sent events support.
- Typed streaming deltas.
- Ergonomic helpers for collecting streamed text.
- Example showing streamed chat text.

## Milestone 3: Embeddings

- OpenAI-compatible embedding requests.
- Typed embedding responses.
- Batch helper methods.
- One-call text embedding helper.

## Milestone 4: Responses API

- OpenAI-compatible `/responses` structures.
- Simple text input helper.
- Multimodal input types where providers support them.
- Structured text format helper.

## Milestone 5: Reliability

- Configurable request timeout.
- Retries for rate limits, server errors, timeouts, and connection failures.
- Conservative retry backoff.

## Milestone 6: Compatibility Notes

- Document provider differences.
- Keep examples for OpenAI-compatible local and hosted APIs.
- Avoid provider lock-in in the core crate.
- Maintain provider preset notes as APIs evolve.
