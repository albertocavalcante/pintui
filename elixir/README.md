# pintui-elixir

Elixir implementation of the pintui design system.

**Status:** Planned

## Planned API

```elixir
defmodule MyApp do
  import Pintui

  def main do
    info("Starting deployment...")

    spinner = spinner("Connecting to server")
    # ... do work ...
    finish_success(spinner, "Connected")

    header("Configuration")
    kv("Environment", "production")
    kv("Region", "us-east-1")

    success("Deployment complete!")
  end
end
```

### With pipes

```elixir
import Pintui

"Connecting to server"
|> spinner()
|> tap(fn _ -> do_connection() end)
|> finish_success("Connected")
```

### With macros

```elixir
import Pintui

info("Starting deployment...")

with_spinner "Connecting to server" do
  connect_to_server()
end

success("Deployment complete!")
```

### Formatting

```elixir
import Pintui.Format

human_size(1_048_576)      # "1.0 MB"
human_duration(5500)       # "5.5s"
pluralize(3, "file")       # "3 files"
```

## Planned Distribution

### mix.exs

```elixir
defp deps do
  [
    {:pintui, "~> 0.1.0"}
  ]
end
```

### Hex.pm

```bash
mix hex.info pintui
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
