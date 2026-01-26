# Contributing

This is a personal project, but PRs are accepted if you find it useful for some reason.

## Bug Reports

Open an issue with:
- Language/implementation affected
- Steps to reproduce
- Expected vs actual behavior

## Code Changes

### Setup

```bash
git clone https://github.com/albertocavalcante/pintui.git
cd pintui
```

### Tests

```bash
# Rust
cd rust && cargo test

# Go
cd go && go test ./...

# Python
cd python && pip install -e ".[dev]" && pytest
```

### Style

- **Rust**: `rustfmt` + `clippy`
- **Go**: `gofmt`
- **Python**: `ruff` + type hints

### Commits

Conventional commits:

```
feat(rust): add new spinner styles
fix(go): handle empty string in human_size
docs: update installation instructions
```

## New Language Implementations

If you want to add a new language:

1. Follow [`spec/tokens.toml`](./spec/tokens.toml)
2. Implement the four modules: messages, layout, progress, format
3. Add tests
4. Include a README

## License

Contributions are MIT licensed.
