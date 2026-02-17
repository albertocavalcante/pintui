//! Checklist output functions for status-line items.
//!
//! This module provides functions for printing status-line items — the most
//! common output pattern in CLI tools. Each item is indented with a colored
//! status icon followed by a message, forming a checklist under a
//! header or section.
//!
//! ## Status Types
//!
//! | Function    | Icon | Color | Use Case                     |
//! |-------------|------|-------|------------------------------|
//! | [`ok`]      | ✓    | Green | Completed / passed           |
//! | [`fail`]    | ✗    | Red   | Failed / error               |
//! | [`skip`]    | ○    | Dim   | Skipped / not applicable     |
//! | [`pending`] | →    | Cyan  | In progress / pending action |
//! | [`item`]    | any  | any   | Custom icon and color        |
//!
//! ## Example
//!
//! ```no_run
//! use pintui::{layout, checklist};
//!
//! layout::section("Dependencies");
//! checklist::ok("tokio 1.40 (up to date)");
//! checklist::ok("serde 1.0 (up to date)");
//! checklist::fail("openssl 1.1 (vulnerable — update to 3.x)");
//! checklist::skip("wasm-bindgen (not applicable)");
//! checklist::pending("rustls 0.23 (downloading...)");
//! // Output:
//! //
//! // Dependencies
//! //   ✓ tokio 1.40 (up to date)
//! //   ✓ serde 1.0 (up to date)
//! //   ✗ openssl 1.1 (vulnerable — update to 3.x)
//! //   ○ wasm-bindgen (not applicable)
//! //   → rustls 0.23 (downloading...)
//! ```

use colored::{Color, Colorize};

use crate::icons;

/// Print a success/completed item with a green ✓ icon.
///
/// Use when an item has completed successfully or passed validation.
///
/// # Example
///
/// ```no_run
/// pintui::checklist::ok("All tests passed");
/// // Output:   ✓ All tests passed
/// ```
pub fn ok(msg: &str) {
    println!("  {} {msg}", icons::ok());
}

/// Print a failure item with a red ✗ icon.
///
/// Use when an item has failed or encountered an error.
///
/// # Example
///
/// ```no_run
/// pintui::checklist::fail("Connection timed out");
/// // Output:   ✗ Connection timed out
/// ```
pub fn fail(msg: &str) {
    println!("  {} {msg}", icons::fail());
}

/// Print a skipped item with a dimmed ○ icon.
///
/// Use when an item was skipped or is not applicable.
///
/// # Example
///
/// ```no_run
/// pintui::checklist::skip("Windows-only check");
/// // Output:   ○ Windows-only check
/// ```
pub fn skip(msg: &str) {
    println!("  {} {msg}", icons::skip());
}

/// Print a pending/in-progress item with a cyan → icon.
///
/// Use when an item is in progress or awaiting action.
///
/// # Example
///
/// ```no_run
/// pintui::checklist::pending("Downloading crate index...");
/// // Output:   → Downloading crate index...
/// ```
pub fn pending(msg: &str) {
    println!("  {} {msg}", icons::arrow());
}

/// Print a checklist item with a custom icon and color.
///
/// Use when the built-in status types don't fit your use case.
/// The icon string is colored with the given [`Color`] and printed
/// with the standard 2-space indent.
///
/// # Example
///
/// ```no_run
/// use colored::Color;
///
/// pintui::checklist::item("★", Color::Yellow, "Featured package");
/// // Output:   ★ Featured package
/// ```
pub fn item(icon: &str, color: Color, msg: &str) {
    println!("  {} {msg}", icon.color(color));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_does_not_panic() {
        ok("test message");
    }

    #[test]
    fn test_fail_does_not_panic() {
        fail("test message");
    }

    #[test]
    fn test_skip_does_not_panic() {
        skip("test message");
    }

    #[test]
    fn test_pending_does_not_panic() {
        pending("test message");
    }

    #[test]
    fn test_item_does_not_panic() {
        item("★", Color::Yellow, "test message");
        item("●", Color::Magenta, "custom item");
    }

    #[test]
    fn test_empty_messages() {
        ok("");
        fail("");
        skip("");
        pending("");
        item("?", Color::White, "");
    }

    #[test]
    fn test_unicode_messages() {
        ok("Hello 世界");
        fail("エラー発生");
        skip("スキップ済み");
        pending("処理中...");
        item("🔥", Color::Red, "Blazing fast");
    }
}
