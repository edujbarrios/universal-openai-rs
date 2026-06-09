# Project Status

`universal-openai-rs` now implements the public API surface that was originally
planned in the roadmap. The project direction is no longer a promise list; it is
an implemented compatibility layer focused on easy, well-structured calls and
simple agent-oriented workflows.

## Implemented

- Models API.
- Legacy completions API.
- Chat completions.
- Prompt-first text generation.
- Native lightweight agent-style workflows, including sequential agent chains.
- Streaming chat completions.
- Typed streaming deltas.
- Testable streaming decoders for OpenAI SSE, lenient SSE, and JSON Lines.
- Text chunk streaming helpers for UI and CLI use.
- Structured output helpers.
- Optional typed structured output schema generation with `structured-output`.
- Tool calling helpers.
- Typed tool execution helpers for lightweight agents.
- Vision and multimodal chat.
- Embeddings.
- Image generation.
- Audio transcription and translation.
- Responses API.
- Typed multimodal Responses API input.
- Responses-first `client.respond(...)` helper.
- Typed Responses output item helpers.
- Files API for upload, list, retrieve, and delete.
- Fine-tuning jobs API.
- Moderations API.
- Cargo feature flags for TLS backend, streaming, multipart, audio, files, and
  structured output.
- Configurable request timeouts.
- Custom `reqwest::Client` injection for production HTTP configuration.
- User-Agent, organization, project, and provider-specific request headers.
- Configurable retries with backoff, jitter, `Retry-After`, and broader method
  coverage.
- Structured API errors with raw body, provider code, parameter, type, and
  request id extraction.
- Redacted `Config` debug output to avoid leaking API keys or custom header
  values in logs.
- Provider presets and custom OpenAI-compatible base URLs.
- Raw compatibility escape hatches for POST, GET, and DELETE.

## Legacy Endpoints

`Engines` is intentionally not first-class because it is a legacy API surface.
Use `send_compatible(...)`, `get_compatible(...)`, or `delete_compatible(...)`
for providers that still expose engines-compatible paths.

## Ongoing Direction

- Improve provider compatibility notes as hosted and local APIs evolve.
- Keep examples small and practical.
- Track known issues and provider quirks in `docs/known-issues.md`.
- Preserve the simple path while adding typed coverage for common workflows.
- Avoid provider lock-in in the core crate.
