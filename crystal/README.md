# pintui-crystal

Crystal implementation of the pintui design system.

**Status:** Planned

## Planned API

```crystal
require "pintui"

# Messages
Pintui.info "Starting deployment..."
Pintui.success "Done!"
Pintui.warn "Config not found, using defaults"
Pintui.error "Connection failed"

# Progress
spinner = Pintui.spinner "Connecting to server"
# ... do work ...
spinner.success "Connected"

# Layout
Pintui.header "Configuration"
Pintui.kv "Environment", "production"
Pintui.kv "Region", "us-east-1"
```

### With blocks

```crystal
require "pintui"

Pintui.info "Starting deployment..."

Pintui.with_spinner "Connecting to server" do
  connect_to_server
end

Pintui.section "Configuration" do |s|
  s.kv "Environment", "production"
  s.kv "Region", "us-east-1"
end

Pintui.success "Deployment complete!"
```

### Include for cleaner syntax

```crystal
require "pintui"
include Pintui::DSL

info "Starting deployment..."
header "Configuration"
kv "Environment", "production"
success "Done!"
```

## Planned Distribution

### shards.yml

```yaml
dependencies:
  pintui:
    github: albertocavalcante/pintui
    branch: main
```

```bash
shards install
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
