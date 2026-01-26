# pintui-nim

Nim implementation of the pintui design system.

**Status:** Planned

## Planned API

```nim
import pintui

# Messages
info "Starting deployment..."
success "Done!"
warn "Config not found, using defaults"
error "Connection failed"

# Progress
let spinner = spinner("Connecting to server")
# ... do work ...
spinner.success "Connected"

# Layout
header "Configuration"
kv "Environment", "production"
kv "Region", "us-east-1"
```

### With templates

```nim
import pintui

info "Starting deployment..."

withSpinner "Connecting to server":
  # ... do work ...
  discard

success "Deployment complete!"
```

### Formatting

```nim
import pintui/format

echo humanSize(1048576)    # "1.0 MB"
echo humanDuration(5500)   # "5.5s"
echo pluralize(3, "file")  # "3 files"
```

## Planned Distribution

### Nimble

```bash
nimble install pintui
```

```nim
# package.nimble
requires "pintui >= 0.1.0"
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
