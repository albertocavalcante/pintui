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

pub mod format;
pub mod layout;
pub mod messages;
pub mod progress;

// Re-export commonly used items at crate root for convenience
pub use format::{human_duration, human_size, parse_size, pluralize, truncate_path};
pub use layout::{blank, divider, header, indent, kv, section, step};
pub use messages::{dim, error, info, success, warn};
pub use progress::{bar, finish_clear, finish_error, finish_success, finish_warn, spinner, StageProgress};
