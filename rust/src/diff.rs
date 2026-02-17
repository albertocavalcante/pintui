//! Diff display helpers for unified-diff-style output.
//!
//! This module provides functions for printing lines in a diff-like format,
//! with colored markers for added, removed, changed, and unchanged (context)
//! lines. Each line is indented with two spaces before the marker icon.
//!
//! ## Line Types
//!
//! | Function    | Marker | Color  | Use Case                         |
//! |-------------|--------|--------|----------------------------------|
//! | [`added`]   | +      | Green  | Newly added line                 |
//! | [`removed`] | -      | Red    | Deleted line                     |
//! | [`changed`] | ~      | Yellow | Modified line                    |
//! | [`context`] | (none) | Dim    | Unchanged surrounding context    |
//!
//! ## Example
//!
//! ```no_run
//! use pintui::diff;
//!
//! diff::added("new_config_line = true");
//! diff::removed("old_config_line = false");
//! diff::changed("value: old → new");
//! diff::context("unchanged line");
//! // Output:
//! //
//! //   + new_config_line = true
//! //   - old_config_line = false
//! //   ~ value: old → new
//! //     unchanged line
//! ```

use colored::Colorize;

use crate::icons;

/// Print an added line with a green `+` prefix.
///
/// The line is printed with a two-space indent, the green `+` icon from
/// [`icons::add()`], and the line text in green.
///
/// # Example
///
/// ```no_run
/// pintui::diff::added("new_config_line = true");
/// // Output:   + new_config_line = true
/// ```
pub fn added(line: &str) {
    println!("  {} {}", icons::add(), line.green());
}

/// Print a removed line with a red `-` prefix.
///
/// The line is printed with a two-space indent, the red `-` icon from
/// [`icons::remove()`], and the line text in red.
///
/// # Example
///
/// ```no_run
/// pintui::diff::removed("old_config_line = false");
/// // Output:   - old_config_line = false
/// ```
pub fn removed(line: &str) {
    println!("  {} {}", icons::remove(), line.red());
}

/// Print a changed line with a yellow `~` prefix.
///
/// The line is printed with a two-space indent, the yellow `~` icon from
/// [`icons::change()`], and the line text in yellow.
///
/// # Example
///
/// ```no_run
/// pintui::diff::changed("value: old → new");
/// // Output:   ~ value: old → new
/// ```
pub fn changed(line: &str) {
    println!("  {} {}", icons::change(), line.yellow());
}

/// Print a context (unchanged) line, dimmed.
///
/// The line is printed with a two-space indent followed by an additional
/// two-space gap (where the icon would be), then the line text in a
/// dimmed style.
///
/// # Example
///
/// ```no_run
/// pintui::diff::context("unchanged line");
/// // Output:     unchanged line
/// ```
pub fn context(line: &str) {
    println!("  {}", format!("  {line}").dimmed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_added_does_not_panic() {
        added("test line");
    }

    #[test]
    fn test_removed_does_not_panic() {
        removed("test line");
    }

    #[test]
    fn test_changed_does_not_panic() {
        changed("test line");
    }

    #[test]
    fn test_context_does_not_panic() {
        context("test line");
    }

    #[test]
    fn test_empty_strings() {
        added("");
        removed("");
        changed("");
        context("");
    }

    #[test]
    fn test_unicode_lines() {
        added("新しい設定 = true");
        removed("古い設定 = false");
        changed("値: 旧 → 新");
        context("変更なし");
    }
}
