# Project Status

`universal-openai-rs` now implements the public API surface that was originally
planned in the roadmap. The project direction is no longer a promise list; it is
an implemented compatibility layer focused on easy, well-structured calls and
simple agent-oriented workflows.

## Implemented

- Core APIs: models, completions, chat, responses, embeddings, images, audio,
  files, fine-tuning, and moderations.
- Simple helpers: `ask`, `prompt`, `embed`, `respond`, default model support,
  and provider presets.
- Agents: lightweight named agents, sequential chains, tool definitions, and
  typed tool execution.
- Streaming: typed deltas, text chunks, OpenAI SSE, lenient SSE, and JSON Lines
  decoders.
- Structured output: JSON object helpers plus optional `structured-output`
  schema generation.
- Production readiness: custom `reqwest::Client`, headers, timeouts, retries,
  `Retry-After`, structured API errors, redacted debug output, and feature flags.
- Compatibility: custom base URLs and raw POST, GET, and DELETE escape hatches.

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
