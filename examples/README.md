# Examples

These examples are intentionally small. The goal is to show the shortest useful
path first, then reveal lower-level OpenAI-compatible control only when needed.

| Example | Shows |
| --- | --- |
| `ask.rs` | One-line text generation with `ask(...)` |
| `simple.rs` | Minimal runnable text generation |
| `prompt_first.rs` | The prompt-first workflow |
| `agents.rs` | Native agent-style workflows |
| `chat.rs` | Full chat completions builder |
| `streaming.rs` | Streaming chat events |
| `vision_chat.rs` | Vision/multimodal chat content |
| `embeddings.rs` | One-call embeddings |
| `responses.rs` | Responses API helper |
| `responses_multimodal.rs` | Multimodal Responses API input |
| `structured_output.rs` | Explicit JSON schema output |
| `typed_json.rs` | Typed JSON parsing into a Rust struct |
| `tool_calling.rs` | Function tool definitions |
| `providers.rs` | Provider presets such as Ollama |
| `production_http.rs` | Custom `reqwest::Client` and provider headers |
| `escape_hatch.rs` | Raw OpenAI-compatible JSON calls |
| `default_model.rs` | `OPENAI_MODEL` and `ask_default(...)` |
| `models.rs` | Model listing |
| `completions.rs` | Legacy text completions |
| `images.rs` | Image generation |
| `audio.rs` | Audio transcription |
| `files.rs` | File upload |
| `fine_tuning.rs` | Fine-tuning job creation |
| `moderations.rs` | Content moderation |

Run an example with:

```bash
cargo run --example ask
```

Set these environment variables first:

```bash
OPENAI_API_KEY=replace-me
OPENAI_MODEL=gpt-4o-mini
```
