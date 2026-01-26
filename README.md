# pintui

**Paint your terminal beautifully** — a cross-language TUI design system.

Like Tailwind for your terminal. Define once, use everywhere.

## The Problem

Every CLI tool you build has inconsistent styling:
- Different colors for success/error
- Different progress spinners
- Different formatting conventions

Your Rust CLI looks different from your Go CLI looks different from your Python script.

## The Solution

Pintui is a **design system** for terminal UIs with implementations in multiple languages:

| Language | Package | Status |
|----------|---------|--------|
| Rust | `pintui` | 🚧 In Progress |
| Go | `pintui-go` | 📋 Planned |
| TypeScript | `@pintui/core` | 📋 Planned |
| Python | `pintui` | 📋 Planned |

All implementations follow the same [design tokens](./spec/tokens.toml), ensuring your tools have a consistent visual language regardless of implementation language.

## Design Tokens

The heart of pintui is [`spec/tokens.toml`](./spec/tokens.toml) — a machine-readable specification defining:

- **Colors**: Semantic color mappings (success=green, error=red, etc.)
- **Icons**: Unicode symbols (✓ ✗ ⚠ ℹ)
- **Spinners**: Animation frames and timing
- **Progress bars**: Characters and templates
- **Layout**: Spacing, indentation, dividers
- **Typography**: Text styling rules

## Quick Example

### Rust

```rust
use pintui::{messages, layout, progress};

messages::info("Starting deployment...");

let spinner = progress::spinner("Connecting to server");
// ... do work ...
progress::finish_success(&spinner, "Connected");

layout::header("Configuration");
layout::kv("Environment", "production");
layout::kv("Region", "us-east-1");

messages::success("Deployment complete!");
```

### Go (planned)

```go
import "github.com/albertocavalcante/pintui-go"

pintui.Info("Starting deployment...")

spinner := pintui.Spinner("Connecting to server")
// ... do work ...
spinner.Success("Connected")

pintui.Header("Configuration")
pintui.KV("Environment", "production")
pintui.KV("Region", "us-east-1")

pintui.Success("Deployment complete!")
```

### TypeScript (planned)

```typescript
import { messages, layout, progress } from '@pintui/core';

messages.info('Starting deployment...');

const spinner = progress.spinner('Connecting to server');
// ... do work ...
spinner.success('Connected');

layout.header('Configuration');
layout.kv('Environment', 'production');
layout.kv('Region', 'us-east-1');

messages.success('Deployment complete!');
```

## Visual Language

Pintui provides opinionated defaults for a unified look:

```
✓ Success messages (green)
✗ Error messages (red)
⚠ Warning messages (yellow)
ℹ Info messages (blue)

⠋ Loading...  (animated spinner)

[━━━━━━━━━━━━━━━━━━━━╸───────────────────] 50/100

Configuration
─────────────
  Environment: production
  Region: us-east-1
```

## Philosophy

1. **Opinionated defaults**: Look good out of the box
2. **Consistent everywhere**: Same visual language across all your tools
3. **Simple API**: Common operations should be one-liners
4. **Spec-driven**: Design tokens are the source of truth

## Project Structure

```
pintui/
├── spec/
│   └── tokens.toml      # Design system specification
├── rust/                # Rust implementation
├── go/                  # Go implementation (planned)
├── typescript/          # TypeScript implementation (planned)
├── python/              # Python implementation (planned)
└── docs/                # Documentation
```

## Name

**Pintui** comes from Portuguese *"pintar"* (to paint) + TUI.

*Paint your terminal beautifully.* 🇧🇷

## License

MIT
