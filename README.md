<div align="center">

# pintui

**Paint your terminal beautifully**

A cross-language design system for terminal UIs.

[![CI](https://github.com/albertocavalcante/pintui/actions/workflows/ci.yml/badge.svg)](https://github.com/albertocavalcante/pintui/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

```
✓ Connected to server                    ℹ Processing 42 files...
✗ Authentication failed                  ⚠ Rate limit: 95%

⠋ Deploying to production...

Configuration
─────────────
  Environment: production
  Region:      us-east-1

[━━━━━━━━━━━━━━━━━━━━╸───────────────────] 50%
```

</div>

---

## What is this?

A personal project to have consistent terminal output across my CLI tools, regardless of what language they're written in.

Same icons, same colors, same spacing — whether it's Rust, Go, or Python.

## Usage

### Rust

```toml
[dependencies]
pintui = "0.1"
```

```rust
use pintui::{messages, layout, progress};

messages::info("Starting deployment...");

let spinner = progress::spinner("Connecting");
// ... work ...
progress::finish_success(&spinner, "Connected");

layout::header("Configuration");
layout::kv("Environment", "production");

messages::success("Done!");
```

### Go

```bash
go get github.com/albertocavalcante/pintui/go
```

```go
import "github.com/albertocavalcante/pintui/go"

pintui.Info("Starting deployment...")

spinner := pintui.Spinner("Connecting")
spinner.Success("Connected")

pintui.Header("Configuration")
pintui.KV("Environment", "production")

pintui.Success("Done!")
```

### Python

```bash
pip install pintui
```

```python
from pintui import messages, layout, progress

messages.info("Starting deployment...")

with progress.spinner("Connecting") as s:
    s.success("Connected")

layout.header("Configuration")
layout.kv("Environment", "production")

messages.success("Done!")
```

## API

| Module | Functions |
|--------|-----------|
| **messages** | `info` `success` `warn` `error` `dim` |
| **layout** | `header` `section` `kv` `step` `divider` `indent` `blank` |
| **progress** | `spinner` `bar` `StageProgress` |
| **format** | `human_size` `human_duration` `pluralize` `truncate_path` `parse_size` |

## Design Tokens

Everything comes from [`spec/tokens.toml`](./spec/tokens.toml):

```toml
[colors]
success = { ansi = "green", hex = "#22c55e" }
error   = { ansi = "red",   hex = "#ef4444" }

[icons]
success = "✓"
error   = "✗"
warning = "⚠"
info    = "ℹ"

[spinner]
frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
```

## Status

| Language | Status |
|----------|--------|
| [Rust](./rust) | ✅ Ready |
| [Go](./go) | ✅ Ready |
| [Python](./python) | ✅ Ready |
| TypeScript | 🔜 Next |

<details>
<summary>Other languages (stubs only)</summary>

Java, Kotlin, Swift, C#, C++, Zig, Ruby, Elixir, Gleam, Crystal, Nim, Odin, OCaml, Haskell, D

</details>

## Name

**pintui** = Portuguese *"pintar"* (to paint) + TUI 🇧🇷

## License

MIT
