//! # pintui
//!
//! **Paint your terminal beautifully** — a TUI design system for Rust.
//!
//! Pintui provides reusable building blocks for CLI interfaces with
//! consistent, beautiful styling out of the box.
//!
//! ## Features
//!
//! - **Messages**: Consistent info, warning, error, and success output
//! - **Layout**: Headers, sections, key-value pairs, step indicators
//! - **Progress**: Spinners, progress bars, multi-stage tracking
//! - **Format**: Human-readable sizes, durations, path truncation
//!
//! ## Quick Start
//!
//! ```no_run
//! use pintui::{messages, layout, progress, format};
//!
//! // Messages
//! messages::info("Starting operation...");
//! messages::success("Done!");
//!
//! // Layout
//! layout::header("Configuration");
//! layout::kv("Version", "1.0.0");
//!
//! // Progress
//! let spinner = progress::spinner("Loading...");
//! // ... do work ...
//! progress::finish_success(&spinner, "Loaded");
//!
//! // Formatting
//! let size = format::human_size(1024 * 1024 * 50);
//! assert_eq!(size, "50.0 MB");
//! ```
//!
//! ## Design Philosophy
//!
//! Pintui provides opinionated defaults for a consistent look:
//!
//! - Success: green checkmark (✓)
//! - Error: red cross (✗)
//! - Warning: yellow warning sign (⚠)
//! - Info: blue info icon (ℹ)
//! - Headers: cyan and bold
//!
//! This creates a unified visual language across all tools using pintui.
//!
//! ## Part of the Pintui Design System
//!
//! This crate is the Rust implementation of the pintui design system.
//! Implementations are also available for Go, TypeScript, and Python,
//! all following the same design tokens for a consistent experience
//! across languages.
//!
//! See: <https://github.com/albertocavalcante/pintui>

pub mod checklist;
pub mod diff;
pub mod dryrun;
pub mod format;
pub mod icons;
pub mod layout;
pub mod list;
pub mod messages;
pub mod progress;
pub mod prompt;
pub mod summary;
pub mod table;
pub mod term;

// Re-export commonly used items at crate root for convenience
pub use format::{
    human_count, human_duration, human_size, parse_size, pluralize, truncate_path,
};
pub use layout::{blank, divider, header, indent, kv, section, step};
pub use messages::{dim, error, info, success, warn};
pub use progress::{
    StageProgress, bar, finish_clear, finish_error, finish_success, finish_warn, spinner,
};
pub use table::{KvGroup, Table};

/// Initialize pintui with automatic environment detection.
///
/// Checks the `NO_COLOR`, `CLICOLOR`, and `CLICOLOR_FORCE` environment
/// variables and configures the `colored` crate accordingly.
///
/// - `NO_COLOR` (any value): disables color output
/// - `CLICOLOR_FORCE` (non-zero): forces color output even if not a TTY
/// - `CLICOLOR=0`: disables color output
///
/// The `colored` crate already respects `NO_COLOR` by default, so calling
/// this function is optional. It's provided for explicitness and for
/// handling `CLICOLOR`/`CLICOLOR_FORCE` as well.
///
/// # Example
///
/// ```no_run
/// pintui::init();
/// ```
pub fn init() {
    // colored v3 already respects NO_COLOR out of the box.
    // Handle CLICOLOR_FORCE and CLICOLOR for completeness.
    if let Ok(val) = std::env::var("CLICOLOR_FORCE") {
        if val != "0" {
            colored::control::set_override(true);
            return;
        }
    }
    if let Ok(val) = std::env::var("CLICOLOR") {
        if val == "0" {
            colored::control::set_override(false);
        }
    }
}

/// Explicitly enable or disable colored output.
///
/// Overrides any environment variable detection. Pass `true` to force
/// colors on, `false` to force them off.
///
/// # Example
///
/// ```no_run
/// pintui::set_color(false); // disable all colors
/// pintui::messages::info("This will be plain text");
/// ```
pub fn set_color(enabled: bool) {
    colored::control::set_override(enabled);
}
