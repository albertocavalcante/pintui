# pintui-swift

Swift implementation of the pintui design system.

**Status:** Planned

## Planned API

```swift
import Pintui

// Messages
Pintui.info("Starting deployment...")
Pintui.success("Done!")
Pintui.warn("Config not found, using defaults")
Pintui.error("Connection failed")

// Progress
let spinner = Pintui.spinner("Connecting to server")
// ... do work ...
spinner.success("Connected")

// Layout
Pintui.header("Configuration")
Pintui.kv("Environment", "production")
Pintui.kv("Region", "us-east-1")
```

### With async/await

```swift
import Pintui

@main
struct MyApp {
    static func main() async throws {
        Pintui.info("Starting deployment...")

        try await Pintui.withSpinner("Connecting to server") {
            try await connect()
        }

        Pintui.success("Deployment complete!")
    }
}
```

## Planned Distribution

### Swift Package Manager

```swift
// Package.swift
dependencies: [
    .package(url: "https://github.com/albertocavalcante/pintui.git", from: "0.1.0")
]
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
