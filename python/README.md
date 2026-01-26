# pintui (Python)

Python implementation of the pintui design system.

**Status:** In Progress (core API complete)

## Installation

```bash
pip install pintui
```

Or install from source:

```bash
pip install -e ".[dev]"
```

## Usage

```python
from pintui import messages, layout, progress, format
from datetime import timedelta

# Messages
messages.info("Starting deployment...")
messages.success("Connected")
messages.warn("Rate limit approaching")
messages.error("Connection failed")
messages.dim("Debug info")

# Formatted messages
messages.infof("Processing {} files", 10)
messages.successf("Deployed to {}", "production")

# Layout
layout.header("Configuration")
layout.section("Server Settings")
layout.kv("Environment", "production")
layout.kv("Region", "us-east-1")
layout.step(1, 3, "Initialize")
layout.blank()
layout.divider(40)
layout.indent(2, "Nested content")

# Progress - Spinner (context manager)
with progress.spinner("Connecting to server") as s:
    # ... do work ...
    s.success("Connected")
# or: s.error("Failed"), s.warn("Timeout"), s.clear()

# Progress - Spinner (direct)
spinner = progress.spinner("Loading")
# ... do work ...
spinner.success("Done")

# Progress - Bar
bar = progress.bar(100, "Downloading")
for i in range(100):
    bar.add(1)
bar.finish()

# Progress - Stages
stages = progress.StageProgress(3)
with stages.next("Compiling") as s:
    # ... work ...
    s.success("Compiled")
with stages.next("Testing") as s:
    s.success("All tests pass")
stages.skip("Deployment")

# Formatting
format.human_size(1024 * 1024 * 100)  # "100.0 MB"
format.human_duration(timedelta(seconds=90))  # "1m 30s"
format.human_duration(5.5)  # "5.5s"
format.pluralize(5, "file", "files")  # "5 files"
format.truncate_path("/very/long/path/file.txt", 15)  # ".../to/file.txt"

size = format.parse_size("100MB")  # 104857600
```

## API Reference

### messages
- `info(msg)` / `infof(fmt, *args)` - Blue info icon
- `success(msg)` / `successf(...)` - Green checkmark
- `warn(msg)` / `warnf(...)` - Yellow warning icon
- `error(msg)` / `errorf(...)` - Red X icon
- `dim(msg)` / `dimf(...)` - Dimmed text

### layout
- `header(title)` - Section header with underline
- `section(title)` - Subsection title
- `kv(key, value)` / `kvf(...)` - Key-value pair
- `step(current, total, msg)` / `stepf(...)` - Step indicator
- `blank()` - Empty line
- `divider(width)` - Horizontal line
- `indent(level, msg)` / `indentf(...)` - Indented text

### progress
- `spinner(msg)` - Returns `SpinnerHandle` (use as context manager or direct)
- `bar(total, description)` - Returns `BarHandle`
- `StageProgress(total)` - Multi-stage progress tracker

### format
- `human_size(bytes)` - Human-readable file size
- `parse_size(str)` - Parse size string to bytes
- `human_duration(duration)` - Human-readable duration (timedelta or seconds)
- `pluralize(count, singular, plural)` - Pluralization
- `truncate_path(path, max_len)` - Truncate long paths

## Dependencies

- [colorama](https://github.com/tartley/colorama) - Cross-platform terminal colors
- [halo](https://github.com/manrajgrover/halo) - Spinners
- [tqdm](https://github.com/tqdm/tqdm) - Progress bars

## Development

```bash
# Create virtual environment
python3 -m venv .venv
source .venv/bin/activate

# Install with dev dependencies
pip install -e ".[dev]"

# Run tests
pytest

# Run linter
ruff check .

# Type check
mypy pintui
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
