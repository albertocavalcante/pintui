# Language Implementations

## Ready

| Language | Install | Tests |
|----------|---------|-------|
| Rust | `cargo add pintui` | 71 |
| Go | `go get github.com/albertocavalcante/pintui/go` | 52 |
| Python | `pip install pintui` | 57 |

## Stubs

These directories exist with planned API examples but no implementation:

TypeScript, Java, Kotlin, Swift, C#, C++, Zig, Ruby, Elixir, Gleam, Crystal, Nim, Odin, OCaml, Haskell, D

## Adding a New Language

1. Read [`spec/tokens.toml`](../spec/tokens.toml)
2. Implement four modules:
   - **messages**: info, success, warn, error, dim
   - **layout**: header, section, kv, step, divider, indent, blank
   - **progress**: spinner, bar, StageProgress
   - **format**: human_size, parse_size, human_duration, pluralize, truncate_path
3. Add tests
4. Write a README
