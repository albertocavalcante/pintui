# pintui (Rust)

```toml
[dependencies]
pintui = "0.1"
```

```rust
use pintui::{messages, layout, progress, format};

// Messages
messages::info("Processing...");
messages::success("Done");
messages::warn("Slow connection");
messages::error("Failed");

// Layout
layout::header("Config");
layout::kv("env", "prod");
layout::step(1, 3, "Building");

// Progress
let spinner = progress::spinner("Loading");
progress::finish_success(&spinner, "Loaded");

let bar = progress::bar(100, "Downloading");
progress::bar_inc(&bar, 50);

// Format
format::human_size(1024 * 1024);  // "1.0 MB"
format::human_duration(Duration::from_secs(90));  // "1m 30s"
```

## Tests

```bash
cargo test
```

71 tests (41 unit + 30 doc tests).
