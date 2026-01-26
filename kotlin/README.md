# pintui-kotlin

Kotlin implementation of the pintui design system.

**Status:** Planned

## Planned API

```kotlin
import io.pintui.messages.*
import io.pintui.layout.*
import io.pintui.progress.*

fun main() {
    info("Starting deployment...")

    spinner("Connecting to server").use { spinner ->
        // ... do work ...
        spinner.success("Connected")
    }

    header("Configuration")
    kv("Environment", "production")
    kv("Region", "us-east-1")

    success("Deployment complete!")
}
```

### With DSL

```kotlin
import io.pintui.dsl.*

fun main() = pintui {
    info("Starting deployment...")

    withSpinner("Connecting to server") {
        // ... do work ...
        success("Connected")
    }

    section("Configuration") {
        kv("Environment", "production")
        kv("Region", "us-east-1")
    }

    success("Deployment complete!")
}
```

## Planned Distribution

- Maven Central: `io.pintui:pintui-kotlin`
- Gradle: `implementation("io.pintui:pintui-kotlin:0.1.0")`

## Contributing

See the main [README](../README.md) for contribution guidelines.
