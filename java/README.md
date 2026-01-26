# pintui-java

Java implementation of the pintui design system.

**Status:** Planned

## Planned API

```java
import io.pintui.Pintui;
import static io.pintui.Messages.*;
import static io.pintui.Layout.*;
import static io.pintui.Progress.*;

public class Example {
    public static void main(String[] args) {
        info("Starting deployment...");

        var spinner = spinner("Connecting to server");
        // ... do work ...
        spinner.success("Connected");

        header("Configuration");
        kv("Environment", "production");
        kv("Region", "us-east-1");

        success("Deployment complete!");
    }
}
```

## Planned Distribution

- Maven Central: `io.pintui:pintui`
- Gradle: `implementation("io.pintui:pintui:0.1.0")`

## Contributing

See the main [README](../README.md) for contribution guidelines.
