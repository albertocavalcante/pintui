//! Summary and stats line helpers for CLI output.
//!
//! This module provides functions for printing summary statistics — single
//! stats or multi-stat lines where each segment is individually colored.
//! These are useful at the end of a command to show counts of items
//! processed, synced, skipped, etc.
//!
//! ## Single Stat
//!
//! ```no_run
//! use colored::Color;
//! use pintui::summary;
//!
//! summary::stat(42, "package", "packages", Color::Cyan);
//! // Output:   42 packages
//! ```
//!
//! ## Multi-Stat Line
//!
//! ```no_run
//! use colored::Color;
//! use pintui::summary;
//!
//! summary::line(&[
//!     (12, "local-only", Color::Cyan),
//!     (8, "external-only", Color::Yellow),
//!     (45, "synced", Color::Green),
//! ]);
//! // Output:   12 local-only, 8 external-only, 45 synced
//! ```

use colored::{Color, Colorize};

/// Print a single stat with a 2-space indent.
///
/// The count and label are colored with the given [`Color`]. The label is
/// chosen automatically: `singular` when `count` is 1, `plural` otherwise.
///
/// # Example
///
/// ```no_run
/// use colored::Color;
///
/// pintui::summary::stat(1, "file", "files", Color::Green);
/// // Output:   1 file
///
/// pintui::summary::stat(42, "package", "packages", Color::Cyan);
/// // Output:   42 packages
/// ```
pub fn stat(count: usize, singular: &str, plural: &str, color: Color) {
    let label = if count == 1 { singular } else { plural };
    println!("  {}", format!("{count} {label}").color(color));
}

/// Print multiple stats on a single line, separated by ", ".
///
/// Each item is rendered as "count label" and colored with its own
/// [`Color`]. The entire line is preceded by a 2-space indent. The label
/// is used as-is (no singular/plural logic).
///
/// If `items` is empty, nothing is printed.
///
/// # Example
///
/// ```no_run
/// use colored::Color;
///
/// pintui::summary::line(&[
///     (12, "local-only", Color::Cyan),
///     (8, "external-only", Color::Yellow),
///     (45, "synced", Color::Green),
/// ]);
/// // Output:   12 local-only, 8 external-only, 45 synced
/// ```
pub fn line(items: &[(usize, &str, Color)]) {
    if items.is_empty() {
        return;
    }

    let segments: Vec<String> = items
        .iter()
        .map(|(count, label, color)| format!("{count} {label}").color(*color).to_string())
        .collect();

    println!("  {}", segments.join(", "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_singular_does_not_panic() {
        stat(1, "file", "files", Color::Green);
    }

    #[test]
    fn test_stat_plural_does_not_panic() {
        stat(0, "file", "files", Color::Green);
        stat(5, "package", "packages", Color::Cyan);
        stat(100, "item", "items", Color::Yellow);
    }

    #[test]
    fn test_stat_zero_does_not_panic() {
        stat(0, "error", "errors", Color::Red);
    }

    #[test]
    fn test_line_single_item_does_not_panic() {
        line(&[(42, "synced", Color::Green)]);
    }

    #[test]
    fn test_line_multiple_items_does_not_panic() {
        line(&[
            (12, "local-only", Color::Cyan),
            (8, "external-only", Color::Yellow),
            (45, "synced", Color::Green),
        ]);
    }

    #[test]
    fn test_line_empty_does_not_panic() {
        line(&[]);
    }

    #[test]
    fn test_empty_labels() {
        stat(5, "", "", Color::White);
        line(&[(0, "", Color::White)]);
    }

    #[test]
    fn test_unicode_labels() {
        stat(3, "パッケージ", "パッケージ", Color::Cyan);
        line(&[
            (1, "ローカル", Color::Cyan),
            (2, "リモート", Color::Yellow),
        ]);
    }
}
