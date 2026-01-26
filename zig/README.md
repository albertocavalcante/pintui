# pintui-zig

Zig implementation of the pintui design system.

**Status:** Planned

## Planned API

```zig
const std = @import("std");
const pintui = @import("pintui");

pub fn main() !void {
    pintui.messages.info("Starting deployment...");

    var spinner = pintui.progress.spinner("Connecting to server");
    defer spinner.deinit();
    // ... do work ...
    spinner.success("Connected");

    pintui.layout.header("Configuration");
    pintui.layout.kv("Environment", "production");
    pintui.layout.kv("Region", "us-east-1");

    pintui.messages.success("Deployment complete!");
}
```

### With Formatting

```zig
const std = @import("std");
const pintui = @import("pintui");

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    pintui.messages.info("Processing {d} files...", .{42});

    const size = pintui.format.humanSize(1024 * 1024 * 50);
    pintui.layout.kv("Size", size);

    pintui.messages.success("Done!");
}
```

## Planned Distribution

### build.zig.zon

```zig
.{
    .name = "myapp",
    .version = "0.1.0",
    .dependencies = .{
        .pintui = .{
            .url = "https://github.com/albertocavalcante/pintui/archive/main.tar.gz",
        },
    },
}
```

### build.zig

```zig
const pintui = b.dependency("pintui", .{
    .target = target,
    .optimize = optimize,
});
exe.addModule("pintui", pintui.module("pintui"));
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
