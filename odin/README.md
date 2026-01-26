# pintui-odin

Odin implementation of the pintui design system.

**Status:** Planned

## Planned API

```odin
package main

import "pintui"

main :: proc() {
    pintui.info("Starting deployment...")

    spinner := pintui.spinner("Connecting to server")
    defer pintui.finish_clear(spinner)
    // ... do work ...
    pintui.finish_success(spinner, "Connected")

    pintui.header("Configuration")
    pintui.kv("Environment", "production")
    pintui.kv("Region", "us-east-1")

    pintui.success("Deployment complete!")
}
```

### Formatting

```odin
package main

import "pintui"
import "pintui/format"

main :: proc() {
    size := format.human_size(1024 * 1024 * 50)
    pintui.kv("Size", size)  // "50.0 MB"

    duration := format.human_duration(5500)
    pintui.kv("Duration", duration)  // "5.5s"
}
```

### Progress bar

```odin
package main

import "pintui"

main :: proc() {
    bar := pintui.bar(100, "Processing")
    for i in 0..<100 {
        // ... do work ...
        pintui.bar_inc(bar, 1)
    }
    pintui.finish_success(bar, "Processed 100 items")
}
```

## Planned Distribution

Add as a git submodule or copy to your project:

```bash
git submodule add https://github.com/albertocavalcante/pintui.git libs/pintui
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
