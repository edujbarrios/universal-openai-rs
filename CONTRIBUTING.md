# Contributing

Thank you for considering a contribution to `universal-openai-rs`.

This project aims to stay:

- OpenAI-compatible in request and response shape.
- Simple for day-to-day Rust usage.
- Provider-agnostic where OpenAI-compatible APIs differ slightly.
- Small enough that contributors can understand the public API quickly.

## Development

Run the standard checks before opening a pull request:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo check --examples
cargo test
cargo doc --no-deps
```

## API Design

Prefer builder methods that make common calls short, but keep the underlying
serialized JSON close to the OpenAI-compatible API specification. When a
provider adds an option that is not part of the core types, use `extra` fields
instead of adding a provider-specific dependency to the main API.
