# Examples

These examples are intentionally small. The goal is to show the shortest useful
path first, then reveal lower-level OpenAI-compatible control only when needed.

| Example | Shows |
| --- | --- |
| `ask.rs` | One-line text generation with `ask(...)` |
| `simple.rs` | Minimal runnable text generation |
| `prompt_first.rs` | The prompt-first workflow |
| `chat.rs` | Full chat completions builder |
| `streaming.rs` | Streaming chat events |
| `embeddings.rs` | One-call embeddings |
| `responses.rs` | Responses API helper |
| `structured_output.rs` | Explicit JSON schema output |
| `typed_json.rs` | Typed JSON parsing into a Rust struct |
| `tool_calling.rs` | Function tool definitions |
| `providers.rs` | Provider presets such as Ollama |
| `escape_hatch.rs` | Raw OpenAI-compatible JSON calls |
| `default_model.rs` | `OPENAI_MODEL` and `ask_default(...)` |

Run an example with:

```bash
cargo run --example ask
```

Set these environment variables first:

```bash
OPENAI_API_KEY=replace-me
OPENAI_MODEL=gpt-4o-mini
```

