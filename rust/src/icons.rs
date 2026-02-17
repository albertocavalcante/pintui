//! Canonical Unicode icon constants and colored convenience functions.
//!
//! This module provides a standard set of icons for use across CLI tools
//! built with pintui. Each icon is available as a raw `&str` constant
//! (for custom coloring) and as a convenience function that returns a
//! [`colored::ColoredString`] with the default color from the pintui
//! design system.
//!
//! ## Quick Start
//!
//! ```
//! use pintui::icons;
//!
//! // Use the colored convenience functions
//! let ok = icons::ok();
//! let fail = icons::fail();
//! println!("{ok} All checks passed");
//! println!("{fail} Linting failed");
//!
//! // Or use the raw constants for custom coloring
//! use colored::Colorize;
//! println!("{} custom style", icons::STAR.magenta().bold());
//! ```
//!
//! ## Available Icons
//!
//! | Constant | Char | Function | Default Color |
//! |----------|------|----------|---------------|
//! | [`OK`] | ✓ | [`ok()`] | Green |
//! | [`FAIL`] | ✗ | [`fail()`] | Red |
//! | [`WARN`] | ⚠ | [`warn()`] | Yellow |
//! | [`INFO`] | ℹ | [`info()`] | Blue |
//! | [`ARROW`] | → | [`arrow()`] | Cyan |
//! | [`SKIP`] | ○ | [`skip()`] | Dimmed |
//! | [`PENDING`] | ● | [`pending()`] | Yellow |
//! | [`STAR`] | ★ | [`star()`] | Green |
//! | [`ADD`] | + | [`add()`] | Green |
//! | [`REMOVE`] | - | [`remove()`] | Red |
//! | [`CHANGE`] | ~ | [`change()`] | Yellow |
//!
//! Constants without a colored function: [`DIAMOND_FILLED`], [`DIAMOND_EMPTY`],
//! [`PLAY`], [`REFRESH`]. Color these with the `colored` crate directly.

use colored::{ColoredString, Colorize};

// =========================================================================
// Raw icon constants
// =========================================================================

/// Checkmark icon: ✓
///
/// Typically used to indicate success or a completed task.
pub const OK: &str = "✓";

/// Cross icon: ✗
///
/// Typically used to indicate failure or a rejected item.
pub const FAIL: &str = "✗";

/// Warning icon: ⚠
///
/// Typically used for warnings or non-fatal issues.
pub const WARN: &str = "⚠";

/// Info icon: ℹ
///
/// Typically used for informational messages.
pub const INFO: &str = "ℹ";

/// Arrow icon: →
///
/// Typically used for navigation, transitions, or pointing to output.
pub const ARROW: &str = "→";

/// Open circle icon: ○
///
/// Typically used for skipped or inactive items.
pub const SKIP: &str = "○";

/// Filled circle icon: ●
///
/// Typically used for pending or in-progress items.
pub const PENDING: &str = "●";

/// Star icon: ★
///
/// Typically used for favorites, highlights, or ratings.
pub const STAR: &str = "★";

/// Filled diamond icon: ◆
///
/// A decorative bullet or marker.
pub const DIAMOND_FILLED: &str = "◆";

/// Empty diamond icon: ◇
///
/// A decorative bullet or marker (outline variant).
pub const DIAMOND_EMPTY: &str = "◇";

/// Play/triangle icon: ▶
///
/// Typically used for "run" or "start" actions.
pub const PLAY: &str = "▶";

/// Refresh/cycle icon: ↻
///
/// Typically used for reload or retry actions.
pub const REFRESH: &str = "↻";

/// Add/plus icon: +
///
/// Typically used for additions in diffs or creation actions.
pub const ADD: &str = "+";

/// Remove/minus icon: -
///
/// Typically used for deletions in diffs or removal actions.
pub const REMOVE: &str = "-";

/// Change/tilde icon: ~
///
/// Typically used for modifications in diffs or edit actions.
pub const CHANGE: &str = "~";

// =========================================================================
// Colored convenience functions
// =========================================================================

/// Return a green checkmark ✓.
///
/// # Example
///
/// ```
/// let icon = pintui::icons::ok();
/// println!("{icon} Build succeeded");
/// ```
pub fn ok() -> ColoredString {
    OK.green()
}

/// Return a red cross ✗.
///
/// # Example
///
/// ```
/// let icon = pintui::icons::fail();
/// println!("{icon} Build failed");
/// ```
pub fn fail() -> ColoredString {
    FAIL.red()
}

/// Return a yellow warning sign ⚠.
///
/// # Example
///
/// ```
/// let icon = pintui::icons::warn();
/// println!("{icon} Deprecated API usage");
/// ```
pub fn warn() -> ColoredString {
    WARN.yellow()
}

/// Return a blue info icon ℹ.
///
/// # Example
///
/// ```
/// let icon = pintui::icons::info();
/// println!("{icon} 42 files indexed");
/// ```
pub fn info() -> ColoredString {
    INFO.blue()
}

/// Return a cyan arrow →.
///
/// # Example
///
/// ```
/// let icon = pintui::icons::arrow();
/// println!("{icon} Next step");
/// ```
pub fn arrow() -> ColoredString {
    ARROW.cyan()
}

/// Return a dimmed open circle ○.
///
/// # Example
///
/// ```
/// let icon = pintui::icons::skip();
/// println!("{icon} Skipped optional check");
/// ```
pub fn skip() -> ColoredString {
    SKIP.dimmed()
}

/// Return a yellow filled circle ●.
///
/// # Example
///
/// ```
/// let icon = pintui::icons::pending();
/// println!("{icon} Waiting for response");
/// ```
pub fn pending() -> ColoredString {
    PENDING.yellow()
}

/// Return a green star ★.
///
/// # Example
///
/// ```
/// let icon = pintui::icons::star();
/// println!("{icon} Featured");
/// ```
pub fn star() -> ColoredString {
    STAR.green()
}

/// Return a green plus +.
///
/// # Example
///
/// ```
/// let icon = pintui::icons::add();
/// println!("{icon} New dependency added");
/// ```
pub fn add() -> ColoredString {
    ADD.green()
}

/// Return a red minus -.
///
/// # Example
///
/// ```
/// let icon = pintui::icons::remove();
/// println!("{icon} Dependency removed");
/// ```
pub fn remove() -> ColoredString {
    REMOVE.red()
}

/// Return a yellow tilde ~.
///
/// # Example
///
/// ```
/// let icon = pintui::icons::change();
/// println!("{icon} Configuration updated");
/// ```
pub fn change() -> ColoredString {
    CHANGE.yellow()
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Constant value tests
    // =========================================================================

    #[test]
    fn test_constants_are_non_empty() {
        let constants: &[&str] = &[
            OK, FAIL, WARN, INFO, ARROW, SKIP, PENDING, STAR,
            DIAMOND_FILLED, DIAMOND_EMPTY, PLAY, REFRESH,
            ADD, REMOVE, CHANGE,
        ];
        for c in constants {
            assert!(!c.is_empty(), "icon constant must not be empty");
        }
    }

    // =========================================================================
    // Colored function tests
    // =========================================================================

    #[test]
    fn test_ok_returns_non_empty() {
        assert!(!ok().to_string().is_empty());
    }

    #[test]
    fn test_fail_returns_non_empty() {
        assert!(!fail().to_string().is_empty());
    }

    #[test]
    fn test_warn_returns_non_empty() {
        assert!(!warn().to_string().is_empty());
    }

    #[test]
    fn test_info_returns_non_empty() {
        assert!(!info().to_string().is_empty());
    }

    #[test]
    fn test_arrow_returns_non_empty() {
        assert!(!arrow().to_string().is_empty());
    }

    #[test]
    fn test_skip_returns_non_empty() {
        assert!(!skip().to_string().is_empty());
    }

    #[test]
    fn test_pending_returns_non_empty() {
        assert!(!pending().to_string().is_empty());
    }

    #[test]
    fn test_star_returns_non_empty() {
        assert!(!star().to_string().is_empty());
    }

    #[test]
    fn test_add_returns_non_empty() {
        assert!(!add().to_string().is_empty());
    }

    #[test]
    fn test_remove_returns_non_empty() {
        assert!(!remove().to_string().is_empty());
    }

    #[test]
    fn test_change_returns_non_empty() {
        assert!(!change().to_string().is_empty());
    }

    // =========================================================================
    // Round-trip tests: function output contains the raw constant
    // =========================================================================

    #[test]
    fn test_colored_output_contains_icon_char() {
        assert!(ok().to_string().contains(OK));
        assert!(fail().to_string().contains(FAIL));
        assert!(warn().to_string().contains(WARN));
        assert!(info().to_string().contains(INFO));
        assert!(arrow().to_string().contains(ARROW));
        assert!(skip().to_string().contains(SKIP));
        assert!(pending().to_string().contains(PENDING));
        assert!(star().to_string().contains(STAR));
        assert!(add().to_string().contains(ADD));
        assert!(remove().to_string().contains(REMOVE));
        assert!(change().to_string().contains(CHANGE));
    }
}
