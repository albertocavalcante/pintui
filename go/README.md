# pintui-go

Go implementation of the pintui design system.

**Status:** In Progress (core API complete)

## Installation

```bash
go get github.com/albertocavalcante/pintui/go
```

## Usage

```go
package main

import "github.com/albertocavalcante/pintui/go"

func main() {
    // Messages
    pintui.Info("Starting deployment...")
    pintui.Success("Connected")
    pintui.Warn("Rate limit approaching")
    pintui.Error("Connection failed")
    pintui.Dim("Debug info")

    // Layout
    pintui.Header("Configuration")
    pintui.Section("Server Settings")
    pintui.KV("Environment", "production")
    pintui.KV("Region", "us-east-1")
    pintui.Step(1, 3, "Initialize")
    pintui.Blank()
    pintui.Divider(40)
    pintui.Indent(2, "Nested content")

    // Progress - Spinner
    spinner := pintui.Spinner("Connecting to server")
    // ... do work ...
    spinner.Success("Connected")
    // or: spinner.Error("Failed"), spinner.Warn("Timeout"), spinner.Clear()

    // Progress - Bar
    bar := pintui.Bar(100, "Downloading")
    for i := 0; i < 100; i++ {
        bar.Add(1)
    }
    bar.Finish()

    // Progress - Stages
    stages := pintui.NewStageProgress(3)
    s1 := stages.Next("Compiling")
    // ... work ...
    s1.Success("Compiled")

    s2 := stages.Next("Testing")
    s2.Success("All tests pass")

    stages.Skip("Deployment (skipped)")

    // Formatting
    pintui.HumanSize(1024*1024*100)  // "100.0 MB"
    pintui.HumanDuration(90*time.Second)  // "1m 30s"
    pintui.Pluralize(5, "file", "files")  // "5 files"
    pintui.TruncatePath("/very/long/path/file.txt", 15)  // ".../to/file.txt"

    size, _ := pintui.ParseSize("100MB")  // 104857600
}
```

## API Reference

### Messages
- `Info(msg)` / `Infof(format, args...)` - Blue info icon
- `Success(msg)` / `Successf(...)` - Green checkmark
- `Warn(msg)` / `Warnf(...)` - Yellow warning icon
- `Error(msg)` / `Errorf(...)` - Red X icon
- `Dim(msg)` / `Dimf(...)` - Dimmed text

### Layout
- `Header(title)` - Section header with underline
- `Section(title)` - Subsection title
- `KV(key, value)` / `KVf(...)` - Key-value pair
- `Step(current, total, msg)` / `Stepf(...)` - Step indicator
- `Blank()` - Empty line
- `Divider(width)` - Horizontal line
- `Indent(level, msg)` / `Indentf(...)` - Indented text

### Progress
- `Spinner(msg)` - Returns `*SpinnerHandle`
- `Bar(total, description)` - Returns `*BarHandle`
- `NewStageProgress(total)` - Returns `*StageProgress`

### Formatting
- `HumanSize(bytes)` - Human-readable file size
- `ParseSize(str)` - Parse size string to bytes
- `HumanDuration(d)` - Human-readable duration
- `Pluralize(count, singular, plural)` - Pluralization
- `TruncatePath(path, maxLen)` - Truncate long paths

## Dependencies

- [github.com/fatih/color](https://github.com/fatih/color) - Terminal colors
- [github.com/briandowns/spinner](https://github.com/briandowns/spinner) - Spinners
- [github.com/schollz/progressbar/v3](https://github.com/schollz/progressbar) - Progress bars

## Contributing

See the main [README](../README.md) for contribution guidelines.
