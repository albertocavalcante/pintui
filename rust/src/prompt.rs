//! Interactive prompt helpers for CLI user input.
//!
//! This module provides thin wrappers around [`dialoguer`] prompts with
//! consistent styling via [`ColorfulTheme`]. Each function blocks until the
//! user provides input and returns the result as a [`std::io::Result`].
//!
//! ## Prompt Types
//!
//! | Function | Widget | Use Case |
//! |----------|--------|----------|
//! | [`confirm`] | Y/n | Yes/no confirmation (defaults to yes) |
//! | [`confirm_default`] | Y/n or y/N | Yes/no with configurable default |
//! | [`select`] | Arrow-key menu | Choose one item from a list |
//! | [`input`] | Free text | Ask for arbitrary text input |
//! | [`input_default`] | Free text | Text input with a pre-filled default |
//!
//! ## Example
//!
//! ```no_run
//! use pintui::prompt;
//!
//! // Confirmation prompt (defaults to yes)
//! if prompt::confirm("Remove 42 packages from local Cellar?").unwrap() {
//!     // proceed
//! }
//!
//! // Selection prompt
//! let choice = prompt::select("Strategy", &["merge", "overwrite", "skip"]).unwrap();
//! println!("You chose index: {choice}");
//!
//! // Free text input
//! let name = prompt::input("Project name").unwrap();
//! println!("Creating project: {name}");
//!
//! // Free text input with a default
//! let branch = prompt::input_default("Branch", "main").unwrap();
//! println!("Using branch: {branch}");
//! ```
//!
//! [`ColorfulTheme`]: dialoguer::theme::ColorfulTheme

use std::io;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};

/// Convert a [`dialoguer::Error`] into a [`std::io::Error`].
fn into_io(err: dialoguer::Error) -> io::Error {
    match err {
        dialoguer::Error::IO(e) => e,
    }
}

/// Ask the user for a yes/no confirmation, defaulting to **yes**.
///
/// Displays a `[Y/n]` prompt using [`dialoguer::Confirm`] with the
/// [`ColorfulTheme`] for consistent styling.
///
/// # Errors
///
/// Returns [`io::Error`] if reading from the terminal fails.
///
/// # Example
///
/// ```no_run
/// if pintui::prompt::confirm("Delete all cached artifacts?").unwrap() {
///     println!("Deleting...");
/// }
/// ```
pub fn confirm(msg: &str) -> io::Result<bool> {
    confirm_default(msg, true)
}

/// Ask the user for a yes/no confirmation with a configurable default.
///
/// When `default` is `true` the prompt shows `[Y/n]`; when `false` it
/// shows `[y/N]`.
///
/// # Errors
///
/// Returns [`io::Error`] if reading from the terminal fails.
///
/// # Example
///
/// ```no_run
/// // Default to "no" for a destructive action
/// if pintui::prompt::confirm_default("Force push to main?", false).unwrap() {
///     println!("Force pushing...");
/// }
/// ```
pub fn confirm_default(msg: &str, default: bool) -> io::Result<bool> {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .default(default)
        .interact()
        .map_err(into_io)
}

/// Show an arrow-key selection menu and return the chosen index.
///
/// The user navigates with arrow keys and confirms with Enter.
/// Returns the zero-based index of the selected item.
///
/// # Errors
///
/// Returns [`io::Error`] if reading from the terminal fails.
///
/// # Example
///
/// ```no_run
/// let items = &["merge", "overwrite", "skip"];
/// let idx = pintui::prompt::select("Strategy", items).unwrap();
/// println!("Selected: {}", items[idx]);
/// ```
pub fn select(prompt: &str, items: &[&str]) -> io::Result<usize> {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()
        .map_err(into_io)
}

/// Ask the user for free-text input.
///
/// The prompt blocks until the user types a value and presses Enter.
///
/// # Errors
///
/// Returns [`io::Error`] if reading from the terminal fails.
///
/// # Example
///
/// ```no_run
/// let name = pintui::prompt::input("Project name").unwrap();
/// println!("Creating {name}...");
/// ```
pub fn input(prompt: &str) -> io::Result<String> {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .map_err(into_io)
}

/// Ask the user for free-text input with a pre-filled default value.
///
/// If the user presses Enter without typing anything, `default` is
/// returned.
///
/// # Errors
///
/// Returns [`io::Error`] if reading from the terminal fails.
///
/// # Example
///
/// ```no_run
/// let branch = pintui::prompt::input_default("Branch", "main").unwrap();
/// println!("Using branch: {branch}");
/// ```
pub fn input_default(prompt: &str, default: &str) -> io::Result<String> {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default.to_string())
        .interact_text()
        .map_err(into_io)
}

#[cfg(test)]
mod tests {
    // Most prompt functions require an interactive TTY and cannot be
    // meaningfully tested in CI. The tests below verify that the
    // dependencies link correctly and that the theme can be constructed.

    #[test]
    fn theme_creation() {
        let _ = dialoguer::theme::ColorfulTheme::default();
    }
}
