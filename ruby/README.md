# pintui-ruby

Ruby implementation of the pintui design system.

**Status:** Planned

## Planned API

```ruby
require 'pintui'

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

```ruby
require 'pintui'

Pintui.info "Starting deployment..."

Pintui.with_spinner "Connecting to server" do
  connect_to_server
end

Pintui.section "Configuration" do
  kv "Environment", "production"
  kv "Region", "us-east-1"
end

Pintui.success "Deployment complete!"
```

### Include for cleaner syntax

```ruby
require 'pintui'
include Pintui::DSL

info "Starting deployment..."
header "Configuration"
kv "Environment", "production"
success "Done!"
```

## Planned Distribution

### RubyGems

```bash
gem install pintui
```

```ruby
# Gemfile
gem 'pintui'
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
