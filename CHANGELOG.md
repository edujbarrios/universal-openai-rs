# Changelog

All notable changes to `universal-openai-rs` will be documented in this file.

The project follows semantic versioning once public releases begin.

## 0.1.0 - Unreleased

### Added

- Intent-first APIs: `ask`, `ask_default`, `ask_json`, and `prompt`.
- OpenAI-compatible chat completions with streaming, tools, structured output,
  and multimodal chat content.
- Responses API with typed text and multimodal input.
- Embeddings, images, audio, files, models, moderations, completions, and
  fine-tuning API surfaces.
- Provider presets for OpenAI, OpenRouter, Groq, Together, Ollama, and custom
  OpenAI-compatible base URLs.
- Raw compatibility escape hatches for POST, GET, and DELETE requests.
- Configurable timeouts and conservative retries for JSON POST requests.
- Examples and tests for core request builders and ergonomic helpers.

