//! Grouped/sectioned list display for CLI output.
//!
//! This module provides a builder-based API for printing grouped lists
//! of items — a common pattern when displaying categorised results,
//! package inventories, or audit summaries. Each group has a bold
//! title and a set of indented items, with automatic empty-group
//! suppression (groups that end up with zero items print nothing at all).
//!
//! ## Quick Start
//!
//! ```no_run
//! use pintui::{list, icons};
//!
//! list::group("Installed packages", |g| {
//!     g.item(icons::ok(), "tokio", "1.40.0");
//!     g.item(icons::ok(), "serde", "1.0.210");
//!     g.item(icons::fail(), "openssl", "vulnerable");
//! });
//!
//! list::group("Skipped", |g| {
//!     g.item_plain("wasm-bindgen (not applicable)");
//! });
//!
//! // If the builder adds no items, nothing is printed:
//! list::group("Empty section", |_g| {
//!     // no items added — this group is silently suppressed
//! });
//! ```
//!
//! ## Output Format
//!
//! ```text
//!   Installed packages
//!     ✓ tokio  1.40.0
//!     ✓ serde  1.0.210
//!     ✗ openssl  vulnerable
//!
//!   Skipped
//!     wasm-bindgen (not applicable)
//! ```

use std::fmt::Display;

use colored::Colorize;

// ---------------------------------------------------------------------------
// Group
// ---------------------------------------------------------------------------

/// A collected group of list items that are printed together.
///
/// Items are accumulated via [`item`](Group::item) and
/// [`item_plain`](Group::item_plain), then rendered when the builder
/// closure passed to [`group`] returns. You do not need to construct
/// this type directly — use the [`group`] function instead.
///
/// # Example
///
/// ```no_run
/// use pintui::{list, icons};
///
/// list::group("Dependencies", |g| {
///     g.item(icons::ok(), "serde", "1.0");
///     g.item_plain("(1 package total)");
/// });
/// ```
pub struct Group {
    /// Accumulated items: `(icon, label, detail)`.
    items: Vec<(String, String, String)>,
}

impl Group {
    /// Create a new, empty group.
    #[must_use]
    const fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add an item with an icon, label, and optional detail.
    ///
    /// The icon may be any type implementing [`Display`] — typically one
    /// of the [`crate::icons`] convenience functions. The detail is
    /// printed dimmed after the label; pass an empty string to omit it.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pintui::{list, icons};
    ///
    /// list::group("Packages", |g| {
    ///     g.item(icons::ok(), "tokio", "1.40.0");
    ///     g.item(icons::fail(), "openssl", "");
    /// });
    /// ```
    pub fn item(&mut self, icon: impl Display, label: &str, detail: &str) {
        self.items
            .push((icon.to_string(), label.to_string(), detail.to_string()));
    }

    /// Add a plain text item without an icon or detail.
    ///
    /// The label is printed with the standard 4-space group indent.
    /// Use this for supplementary notes or summary lines within a group.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pintui::list;
    ///
    /// list::group("Notes", |g| {
    ///     g.item_plain("No issues found.");
    /// });
    /// ```
    pub fn item_plain(&mut self, label: &str) {
        self.items
            .push((String::new(), label.to_string(), String::new()));
    }

    /// Print the group title and all collected items.
    ///
    /// If no items were added, nothing is printed (empty-group
    /// suppression).
    fn print(&self, title: &str) {
        if self.items.is_empty() {
            return;
        }

        // Title: bold, 2-space indent.
        println!("  {}", title.bold());

        // Items: 4-space indent.
        for (icon, label, detail) in &self.items {
            if icon.is_empty() && detail.is_empty() {
                // Plain item — just the label.
                println!("    {label}");
            } else if detail.is_empty() {
                // Icon + label, no detail.
                println!("    {icon} {label}");
            } else {
                // Icon + label + dimmed detail.
                println!("    {icon} {label}  {}", detail.dimmed());
            }
        }

        // Trailing blank line for spacing between groups.
        println!();
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Print a grouped list with a bold section title.
///
/// The `builder` closure receives a mutable reference to a [`Group`].
/// Add items inside the closure with [`Group::item`] and
/// [`Group::item_plain`]. When the closure returns, the group is
/// rendered to stdout.
///
/// If the builder adds no items, **nothing is printed** — this is the
/// empty-group suppression behaviour that keeps output clean when a
/// category has no entries.
///
/// # Example
///
/// ```no_run
/// use pintui::{list, icons};
///
/// let local_only = vec!["foo", "bar"];
///
/// list::group("Local only (not yet stashed)", |g| {
///     for pkg in &local_only {
///         g.item(icons::ok(), pkg, "1.2 MB");
///     }
/// });
/// ```
pub fn group(title: &str, builder: impl FnOnce(&mut Group)) {
    let mut g = Group::new();
    builder(&mut g);
    g.print(title);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_group_prints_nothing() {
        // An empty group should not panic and should produce no output.
        // (We cannot easily capture stdout in a unit test, but we verify
        // the items vec is empty and print() returns without error.)
        let g = Group::new();
        assert!(g.items.is_empty());
        g.print("Should not appear");
    }

    #[test]
    fn group_with_items_does_not_panic() {
        group("Test Group", |g| {
            g.item("✓", "first", "detail-a");
            g.item("✗", "second", "detail-b");
            g.item("★", "third", "");
        });
    }

    #[test]
    fn item_plain_does_not_panic() {
        group("Plain Items", |g| {
            g.item_plain("just a line");
            g.item_plain("another line");
        });
    }

    #[test]
    fn group_accumulates_items() {
        let mut g = Group::new();
        assert_eq!(g.items.len(), 0);

        g.item("✓", "alpha", "100 KB");
        assert_eq!(g.items.len(), 1);

        g.item("✗", "beta", "");
        assert_eq!(g.items.len(), 2);

        g.item_plain("gamma");
        assert_eq!(g.items.len(), 3);
    }

    #[test]
    fn empty_group_via_public_api() {
        // The public `group` function should silently suppress output.
        group("Empty", |_g| {});
    }

    #[test]
    fn mixed_items_do_not_panic() {
        group("Mixed", |g| {
            g.item("✓", "with icon and detail", "1.0 MB");
            g.item("✗", "with icon no detail", "");
            g.item_plain("plain text only");
        });
    }

    #[test]
    fn unicode_content() {
        group("日本語グループ", |g| {
            g.item("✓", "パッケージ", "1.0 MB");
            g.item_plain("テスト完了");
        });
    }
}
