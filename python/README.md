# pintui (Python)

Python implementation of the pintui design system.

**Status:** Planned

## Planned API

```python
import pintui
from pintui import messages, layout, progress

messages.info("Starting deployment...")

with progress.spinner("Connecting to server") as spinner:
    # ... do work ...
    spinner.success("Connected")

layout.header("Configuration")
layout.kv("Environment", "production")
layout.kv("Region", "us-east-1")

messages.success("Deployment complete!")
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
