# pintui-go

Go implementation of the pintui design system.

**Status:** Planned

## Planned API

```go
package main

import "github.com/albertocavalcante/pintui-go"

func main() {
    pintui.Info("Starting deployment...")

    spinner := pintui.Spinner("Connecting to server")
    // ... do work ...
    spinner.Success("Connected")

    pintui.Header("Configuration")
    pintui.KV("Environment", "production")
    pintui.KV("Region", "us-east-1")

    pintui.Success("Deployment complete!")
}
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
