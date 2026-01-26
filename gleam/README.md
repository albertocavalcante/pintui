# pintui-gleam

Gleam implementation of the pintui design system.

**Status:** Planned

## Planned API

```gleam
import pintui/messages
import pintui/layout
import pintui/progress

pub fn main() {
  messages.info("Starting deployment...")

  let spinner = progress.spinner("Connecting to server")
  // ... do work ...
  progress.finish_success(spinner, "Connected")

  layout.header("Configuration")
  layout.kv("Environment", "production")
  layout.kv("Region", "us-east-1")

  messages.success("Deployment complete!")
}
```

### With use for cleanup

```gleam
import pintui/messages
import pintui/progress

pub fn main() {
  messages.info("Starting deployment...")

  use spinner <- progress.with_spinner("Connecting to server")
  // ... do work ...
  // spinner automatically finished

  messages.success("Deployment complete!")
}
```

### Formatting

```gleam
import pintui/format

let size = format.human_size(1_048_576)
// "1.0 MB"

let duration = format.human_duration(milliseconds: 5500)
// "5.5s"
```

## Planned Distribution

### gleam.toml

```toml
[dependencies]
pintui = "~> 0.1"
```

### Hex.pm

```bash
gleam add pintui
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
