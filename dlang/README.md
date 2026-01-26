# pintui-d

D implementation of the pintui design system.

**Status:** Planned

## Planned API

```d
import pintui;

void main() {
    info("Starting deployment...");

    auto spinner = spinner("Connecting to server");
    scope(exit) spinner.clear();
    // ... do work ...
    spinner.success("Connected");

    header("Configuration");
    kv("Environment", "production");
    kv("Region", "us-east-1");

    success("Deployment complete!");
}
```

### With scope guards

```d
import pintui;

void main() {
    info("Starting deployment...");

    {
        auto spinner = scopedSpinner("Connecting to server");
        // ... do work ...
        // automatically finishes on scope exit
    }

    success("Deployment complete!");
}
```

### Formatting

```d
import pintui.format;

void main() {
    writeln(humanSize(1048576));      // "1.0 MB"
    writeln(humanDuration(5500));     // "5.5s"
    writeln(pluralize(3, "file"));    // "3 files"
}
```

### With UFCS (Uniform Function Call Syntax)

```d
import pintui;

void main() {
    "Starting deployment...".info();
    "Configuration".header();
    kv("Environment", "production");
    "Deployment complete!".success();
}
```

## Planned Distribution

### dub.json

```json
{
    "dependencies": {
        "pintui": "~>0.1.0"
    }
}
```

### dub.sdl

```sdl
dependency "pintui" version="~>0.1.0"
```

```bash
dub add pintui
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
