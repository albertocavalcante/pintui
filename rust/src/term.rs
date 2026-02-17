//! Terminal utilities for detecting size and capabilities.
//!
//! This module provides functions for querying the current terminal
//! dimensions, with sensible fallbacks when detection is not possible
//! (e.g., piped output or environments without a tty).
//!
//! ## Example
//!
//! ```no_run
//! use pintui::term;
//!
//! let w = term::width();
//! let h = term::height();
//! println!("Terminal is {w} columns x {h} rows");
//! ```

use terminal_size::{Height, Width};

/// Default terminal width (columns) used when detection fails.
pub const DEFAULT_WIDTH: usize = 80;

/// Default terminal height (rows) used when detection fails.
pub const DEFAULT_HEIGHT: usize = 24;

/// Return the current terminal width in columns.
///
/// Queries the terminal for its current width. If detection fails
/// (e.g., stdout is piped, no tty is attached), returns
/// [`DEFAULT_WIDTH`] (80 columns).
///
/// # Example
///
/// ```no_run
/// let cols = pintui::term::width();
/// assert!(cols > 0);
/// ```
pub fn width() -> usize {
    terminal_size::terminal_size()
        .map_or(DEFAULT_WIDTH, |(Width(w), _)| w as usize)
}

/// Return the current terminal height in rows.
///
/// Queries the terminal for its current height. If detection fails
/// (e.g., stdout is piped, no tty is attached), returns
/// [`DEFAULT_HEIGHT`] (24 rows).
///
/// # Example
///
/// ```no_run
/// let rows = pintui::term::height();
/// assert!(rows > 0);
/// ```
pub fn height() -> usize {
    terminal_size::terminal_size()
        .map_or(DEFAULT_HEIGHT, |(_, Height(h))| h as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_returns_positive_value() {
        assert!(width() > 0);
    }

    #[test]
    fn height_returns_positive_value() {
        assert!(height() > 0);
    }

    #[test]
    fn default_width_is_80() {
        assert_eq!(DEFAULT_WIDTH, 80);
    }

    #[test]
    fn default_height_is_24() {
        assert_eq!(DEFAULT_HEIGHT, 24);
    }
}
