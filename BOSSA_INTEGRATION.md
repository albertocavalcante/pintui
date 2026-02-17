# pintui — Feature Requests from bossa

Comprehensive audit of what bossa needs from pintui, based on actual usage
patterns across 29 source files that import `colored::Colorize` directly,
bypassing pintui entirely.

---

## P0 — High Impact, High Volume

### 1. Checklist / Status Line Items

**The problem:** bossa has **92** instances of `"✓".green()` and **38** of
`"✗".red()` hand-formatted across 20+ files. Every command builds its own
checklist display from raw colored strings.

**Current bossa code (repeated everywhere):**

```rust
// doctor.rs, caches.rs, stow.rs, cellar.rs, dotfiles.rs, ...
println!("  {} {}", "✓".green(), description);
println!("  {} {}", "✗".red(), description);
println!("  {} {}", "○".yellow(), "skipped");
println!("  {} {}", "→".cyan(), "Would trim foo (4.2 MB)");
```

**Requested API:**

```rust
use pintui::checklist;

checklist::ok("Git installed");              // ✓ Git installed
checklist::fail("Node missing");             // ✗ Node missing
checklist::skip("Docker (not needed)");      // ○ Docker (not needed)
checklist::pending("Checking brew...");       // → Checking brew...
checklist::item("★", Color::Green, "Essential package");  // custom icon
```

This is pintui's single biggest gap — the `messages` module handles
standalone messages, but bossa's primary output pattern is **lists of
items with status indicators**, not standalone messages.

---

### 2. Table Formatting

**The problem:** 39 instances of `{:<N}` manual column alignment across
9 files. Every status/list command hand-pads columns.

**Current bossa code:**

```rust
// cellar.rs
println!("    {} {:<30} {}", marker, pkg, ui::format_size(size).dimmed());

// dotfiles.rs
println!("    {:<30} {}", path.dimmed(), status_str);

// theme.rs
println!("  {:<20} {:<15} {}", name, kind, path.dimmed());

// caches.rs
println!("  {:<40} → {}", source.dimmed(), target.display());
```

**Requested API:**

```rust
use pintui::table;

// Simple two-column (auto-sized from content)
let mut t = table::new();
t.row("Package", "Status");
t.row("git", "installed");
t.row("neovim", "missing");
t.print();

// Three-column with alignment hints
let mut t = table::aligned(&[30, 15, 0]); // 0 = auto/rest
t.row(&["Name", "Kind", "Path"]);
t.row(&["gruvbox", "theme", "~/.config/bat/themes"]);
t.print();

// Or a one-liner for key-value status lines
table::status("git", "✓", Color::Green, "3.2 MB");
```

No need for full ASCII-box tables — just consistent column alignment with
optional coloring. The existing `kv()` function handles single key-value
pairs, but bossa needs aligned **lists** of key-value pairs.

---

### 3. Icon / Marker Constants

**The problem:** 18+ unique Unicode icons are hardcoded as string literals
across the codebase with ad hoc coloring. No single source of truth.

**Icons used in bossa today:**

| Icon | Color   | Used In                           | Meaning          |
|------|---------|-----------------------------------|------------------|
| ✓    | green   | 20 files, 92 uses                 | success/present  |
| ✗    | red     | 15 files, 38 uses                 | error/missing    |
| ⚠    | yellow  | messages module only              | warning          |
| ℹ    | blue    | messages module only              | info             |
| →    | cyan    | cellar, caches, engine            | action/dry-run   |
| ○    | yellow  | doctor, caches, nova, cellar      | skipped/optional |
| ●    | yellow  | cellar                            | external-only    |
| ★    | green   | cellar                            | essential        |
| ◆    | green   | dotfiles_reconcile                | both sources     |
| ◇    | dimmed  | dotfiles_reconcile                | one source       |
| ▶    | blue    | nova                              | stage running    |
| ↻    | yellow  | engine                            | changed          |
| +    | green   | engine/differ                     | added            |
| -    | red     | engine/differ                     | removed          |
| ~    | yellow  | engine/differ                     | modified         |

**Requested API:**

```rust
use pintui::icons;

// Constants (colored strings ready to print)
icons::OK       // "✓".green()
icons::FAIL     // "✗".red()
icons::WARN     // "⚠".yellow()
icons::INFO     // "ℹ".blue()
icons::ARROW    // "→".cyan()
icons::SKIP     // "○".dimmed()
icons::PENDING  // "●".yellow()
icons::STAR     // "★".green()
icons::ADD      // "+".green()
icons::REMOVE   // "-".red()
icons::CHANGE   // "~".yellow()

// Or as a function if ColoredString lifetime is tricky
icons::ok()     -> ColoredString
```

This gives pintui a canonical icon vocabulary that all consumers share.

---

## P1 — Medium Impact

### 4. Dry-Run Output Mode

**The problem:** bossa has a recurring pattern for `--dry-run` across
cellar, caches, stow commands: show what *would* happen with different
formatting, then print a "Dry run — no changes made" footer.

**Current bossa code:**

```rust
if dry_run {
    println!("  {} Would trim {} ({})", "→".cyan(), pkg, size);
} else {
    println!("  {} Trimmed {} ({})", "✓".green(), pkg, size);
}
// ... later ...
if dry_run {
    ui::warn("Dry run — no changes made");
}
```

**Requested API:**

```rust
use pintui::dryrun;

// Wraps output to automatically use → prefix and (dry run) suffix
dryrun::action("Would trim", &format!("{pkg} ({size})"));
// Output: → Would trim git (4.2 MB)

dryrun::footer();
// Output: ⚠ Dry run — no changes made
```

Or a mode toggle that changes `checklist::ok()` to print `→` instead of `✓`
when dry-run is active.

---

### 5. Summary / Stats Line

**The problem:** bossa frequently builds summary lines with counts and
sizes that need consistent formatting with color highlights.

**Current bossa code:**

```rust
println!(
    "  {} packages to trim (keeping {} essential)",
    candidates.len().to_string().bold(),
    keep_set.len().to_string().green()
);

ui::success(&format!(
    "Trimmed {} packages, reclaimed {}",
    candidates.len(),
    ui::format_size(total_reclaimed)
));

println!(
    "  {} local-only, {} external-only, {} both",
    local_only.len().to_string().cyan(),
    ext_only.len().to_string().yellow(),
    both.len().to_string().green()
);
```

**Requested API:**

```rust
use pintui::summary;

// Counts with automatic coloring and pluralization
summary::stat(42, "package", "packages", Color::Cyan);
// Output: 42 packages (cyan)

summary::line(&[
    (local_only.len(), "local-only", Color::Cyan),
    (ext_only.len(), "external-only", Color::Yellow),
    (both.len(), "synced", Color::Green),
]);
// Output: 12 local-only, 8 external-only, 45 synced
```

---

### 6. NO_COLOR / CLICOLOR Support

**The problem:** neither pintui nor bossa respects the `NO_COLOR`
environment variable (https://no-color.org/). The `colored` crate
supports this via `colored::control::set_override(false)`, but pintui
doesn't expose or enforce it.

**Requested:** pintui should check `NO_COLOR` at init time and disable
colors automatically. Provide an explicit override too:

```rust
pintui::init(); // auto-detect NO_COLOR, CLICOLOR, CLICOLOR_FORCE

// Or manual
pintui::set_color(false);
```

---

### 7. `kv()` with Right-Aligned / Fixed-Width Keys

**The problem:** the current `kv("Key", "Value")` prints `Key: Value`,
but when displaying multiple kv pairs, the keys have different lengths
and values don't align.

**Current output:**

```
  Local Cellar:    /opt/homebrew/Cellar (3.2 GB)
  External Cellar: /Volumes/T9/homebrew/Cellar (3.2 GB)
  Essential list:  10 packages
```

This alignment is done manually with spaces inside the key string.

**Requested API:**

```rust
// Auto-align a group of kv pairs
let mut kvs = layout::kv_group();
kvs.add("Local Cellar", "/opt/homebrew/Cellar (3.2 GB)");
kvs.add("External Cellar", "/Volumes/T9/homebrew/Cellar (3.2 GB)");
kvs.add("Essential list", "10 packages");
kvs.print(); // keys right-padded to longest key width
```

---

## P2 — Nice to Have

### 8. Diff Display Helpers

**The problem:** `dotfiles_reconcile.rs` and `engine/differ.rs` build
diff output with `+`/`-`/`~` markers and colored text. This is a
common CLI pattern.

**Requested API:**

```rust
use pintui::diff;

diff::added("new_config_line = true");    // + new_config_line = true (green)
diff::removed("old_config_line = false"); // - old_config_line = false (red)
diff::changed("value: old → new");        // ~ value: old → new (yellow)
diff::context("unchanged line");          //   unchanged line (dimmed)
```

---

### 9. Terminal Width Awareness

**The problem:** `divider(width)` requires the caller to know the
terminal width. Truncation and table formatting also benefit from
knowing the available width.

**Requested API:**

```rust
pintui::term::width() -> usize  // returns terminal width or 80 fallback

layout::divider_full();  // fills terminal width
```

---

### 10. Confirmation Prompt

**The problem:** bossa doesn't currently have confirmation prompts, but
several commands (trim, demote, disk repartition) would benefit from
them. Currently `--yes` flags bypass the need, but interactive mode
should ask.

**Requested API:**

```rust
use pintui::prompt;

if prompt::confirm("Remove 42 packages from local Cellar?")? {
    // proceed
}

let choice = prompt::select("Strategy", &["merge", "overwrite", "skip"])?;
```

`indicatif` is already a dependency; `dialoguer` or a small built-in
would complement it.

---

### 11. Grouped/Sectioned Lists

**The problem:** `bossa doctor`, `bossa caches status`, and
`bossa cellar status` all display items in groups with headers.

**Current bossa code:**

```rust
println!("{}", "  Local only (not yet stashed):".bold());
for pkg in &local_only {
    println!("    {} {:<30} {}", marker, pkg, size.dimmed());
}
println!();
println!("{}", "  External only (trimmed locally):".bold());
for pkg in &ext_only {
    println!("    {} {:<30} {}", "●".yellow(), pkg, size.dimmed());
}
```

**Requested API:**

```rust
use pintui::list;

list::group("Local only (not yet stashed)", |g| {
    for pkg in &local_only {
        g.item(icons::ok(), pkg, &format_size(size));
    }
});
// Handles indentation, spacing, empty-group suppression automatically
```

---

### 12. `format::human_count`

**The problem:** bossa frequently formats counts with `.to_string()`
then chains `.bold()` or `.green()`. A helper that does
`1,234` comma-separated formatting would be nice for large counts.

**Requested API:**

```rust
format::human_count(1234)    // "1,234"
format::human_count(42)      // "42"
format::human_count(1000000) // "1,000,000"
```

---

## Bugs / Inconsistencies

### B1. `messages::dim()` adds indentation, others don't

`dim()` prints `"  {msg.dimmed()}"` (2 spaces), while `info()`,
`success()`, `warn()`, `error()` print at column 0 with icon prefix.
This makes `dim()` behave differently from the other message functions —
callers can't predict the indentation.

**Suggestion:** either remove the hard-coded indent from `dim()` (let
callers control it) or document the intent clearly.

### B2. `kv()` key is dimmed, value is plain — inverted from convention

Most CLI tools dim the *value* or keep both normal, while `kv()` dims
the key. This makes keys harder to scan. Consider making the key bold
or normal, and optionally dimming the value.

### B3. `header()` underline width uses byte length, not display width

```rust
println!("{}", "─".repeat(title.len()).dimmed());
```

For Unicode titles (e.g., CJK characters), `title.len()` returns byte
count, not display width. A title like "日本語" is 9 bytes but 6
display columns. Use `unicode-width` crate for correct measurement.

---

## Summary

| Priority | Item                         | bossa Impact            |
|----------|------------------------------|-------------------------|
| P0       | Checklist / status items     | 92+ `"✓".green()` uses  |
| P0       | Table formatting             | 39 manual `{:<N}` pads  |
| P0       | Icon constants               | 18 ad-hoc icons         |
| P1       | Dry-run output mode          | 3 commands with dry-run |
| P1       | Summary/stats line           | 10+ manual stat lines   |
| P1       | NO_COLOR support             | 0/29 files respect it   |
| P1       | kv() alignment groups        | 5+ manual padded groups |
| P2       | Diff display helpers         | 2 diff-style commands   |
| P2       | Terminal width awareness      | divider, truncate, table|
| P2       | Confirmation prompts         | 4+ commands could use   |
| P2       | Grouped/sectioned lists      | 5+ status commands      |
| P2       | human_count formatter        | frequent count display  |
| Bug      | dim() inconsistent indent    | confusing for callers   |
| Bug      | kv() key dimmed (inverted)   | hard to scan            |
| Bug      | header() wrong Unicode width | broken for CJK titles   |
