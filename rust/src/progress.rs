//! Progress indicators for long-running operations.
//!
//! This module provides spinners and progress bars for giving users
//! feedback during operations that take time.
//!
//! ## Spinner (Indeterminate Progress)
//!
//! Use when you don't know how long an operation will take:
//!
//! ```no_run
//! use pintui::progress;
//!
//! let spinner = progress::spinner("Connecting to server...");
//! // ... do work ...
//! progress::finish_success(&spinner, "Connected");
//! ```
//!
//! ## Progress Bar (Determinate Progress)
//!
//! Use when you know the total amount of work:
//!
//! ```no_run
//! use pintui::progress;
//!
//! let bar = progress::bar(100, "Downloading");
//! for i in 0..100 {
//!     // ... do work ...
//!     bar.inc(1);
//! }
//! progress::finish_success(&bar, "Downloaded");
//! ```
//!
//! ## Multi-Stage Progress
//!
//! Use for operations with multiple distinct phases:
//!
//! ```no_run
//! use pintui::progress::StageProgress;
//!
//! let mut stages = StageProgress::new(3);
//!
//! let s1 = stages.next("Fetching");
//! // ... fetch ...
//! pintui::progress::finish_success(&s1, "Fetched");
//!
//! let s2 = stages.next("Building");
//! // ... build ...
//! pintui::progress::finish_success(&s2, "Built");
//! ```

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Create a spinner for indeterminate progress.
///
/// The spinner animates while work is in progress, showing the user
/// that the application hasn't frozen.
///
/// # Example
///
/// ```no_run
/// let spinner = pintui::progress::spinner("Loading configuration...");
/// // ... do work ...
/// pintui::progress::finish_success(&spinner, "Configuration loaded");
/// ```
pub fn spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// Create a progress bar for determinate progress.
///
/// Use when you know the total amount of work to be done.
///
/// # Arguments
///
/// * `len` - Total number of units of work
/// * `prefix` - Label shown before the progress bar
///
/// # Example
///
/// ```no_run
/// let bar = pintui::progress::bar(100, "Processing");
/// for _ in 0..100 {
///     // ... do work ...
///     bar.inc(1);
/// }
/// pintui::progress::finish_success(&bar, "Processed 100 items");
/// ```
pub fn bar(len: u64, prefix: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{prefix:.bold.dim} {spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {wide_msg}",
            )
            .unwrap()
            .progress_chars("━━╸"),
    );
    pb.set_prefix(prefix.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Finish a progress indicator with a success message.
///
/// Replaces the spinner/bar with a green checkmark and message.
///
/// # Example
///
/// ```no_run
/// let spinner = pintui::progress::spinner("Working...");
/// // ... do work ...
/// pintui::progress::finish_success(&spinner, "Work complete");
/// // Output: ✓ Work complete
/// ```
pub fn finish_success(pb: &ProgressBar, msg: &str) {
    pb.set_style(ProgressStyle::default_spinner().template("{msg}").unwrap());
    pb.finish_with_message(format!("{} {}", "✓".green(), msg));
}

/// Finish a progress indicator with an error message.
///
/// Replaces the spinner/bar with a red cross and message.
///
/// # Example
///
/// ```no_run
/// let spinner = pintui::progress::spinner("Connecting...");
/// // ... connection fails ...
/// pintui::progress::finish_error(&spinner, "Connection failed");
/// // Output: ✗ Connection failed
/// ```
pub fn finish_error(pb: &ProgressBar, msg: &str) {
    pb.set_style(ProgressStyle::default_spinner().template("{msg}").unwrap());
    pb.finish_with_message(format!("{} {}", "✗".red(), msg));
}

/// Finish a progress indicator with a warning message.
///
/// Replaces the spinner/bar with a yellow warning sign and message.
///
/// # Example
///
/// ```no_run
/// let spinner = pintui::progress::spinner("Checking...");
/// // ... partial success ...
/// pintui::progress::finish_warn(&spinner, "Completed with warnings");
/// // Output: ⚠ Completed with warnings
/// ```
pub fn finish_warn(pb: &ProgressBar, msg: &str) {
    pb.set_style(ProgressStyle::default_spinner().template("{msg}").unwrap());
    pb.finish_with_message(format!("{} {}", "⚠".yellow(), msg));
}

/// Finish a progress indicator and clear it from display.
///
/// Use when you want to remove the progress indicator without
/// leaving a final message.
pub fn finish_clear(pb: &ProgressBar) {
    pb.finish_and_clear();
}

/// Multi-stage progress tracker.
///
/// Tracks progress through a sequence of named stages, showing
/// `[current/total]` indicators.
///
/// # Example
///
/// ```no_run
/// use pintui::progress::{StageProgress, finish_success};
///
/// let mut stages = StageProgress::new(3);
///
/// let s1 = stages.next("Downloading");
/// // ... download ...
/// finish_success(&s1, "Downloaded");
///
/// let s2 = stages.next("Extracting");
/// // ... extract ...
/// finish_success(&s2, "Extracted");
///
/// let s3 = stages.next("Installing");
/// // ... install ...
/// finish_success(&s3, "Installed");
/// ```
pub struct StageProgress {
    current: usize,
    total: usize,
}

impl StageProgress {
    /// Create a new stage progress tracker.
    ///
    /// # Arguments
    ///
    /// * `total` - Total number of stages
    #[must_use]
    pub const fn new(total: usize) -> Self {
        Self { current: 0, total }
    }

    /// Start the next stage and return a spinner for it.
    ///
    /// The spinner shows `[current/total]` before the stage name.
    pub fn next(&mut self, name: &str) -> ProgressBar {
        self.current += 1;
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template(&format!(
                    "{{spinner:.cyan}} [{}/{}] {{msg}}",
                    self.current, self.total
                ))
                .unwrap(),
        );
        pb.set_message(name.to_string());
        pb.enable_steady_tick(Duration::from_millis(80));
        pb
    }

    /// Skip a stage, printing a skip indicator.
    ///
    /// Increments the stage counter but doesn't create a spinner.
    pub fn skip(&mut self, name: &str) {
        self.current += 1;
        println!(
            "  {} [{}/{}] {} (skipped)",
            "○".dimmed(),
            self.current,
            self.total,
            name
        );
    }

    /// Get the current stage number (1-indexed).
    #[must_use]
    pub const fn current(&self) -> usize {
        self.current
    }

    /// Get the total number of stages.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.total
    }

    /// Check if all stages are complete.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.current >= self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_creation() {
        let pb = spinner("Test");
        pb.finish_and_clear();
    }

    #[test]
    fn test_bar_creation() {
        let pb = bar(100, "Test");
        pb.finish_and_clear();
    }

    #[test]
    fn test_finish_functions() {
        let pb = spinner("Test");
        finish_success(&pb, "Done");

        let pb = spinner("Test");
        finish_error(&pb, "Failed");

        let pb = spinner("Test");
        finish_warn(&pb, "Warning");

        let pb = spinner("Test");
        finish_clear(&pb);
    }

    #[test]
    fn test_stage_progress() {
        let mut stages = StageProgress::new(3);

        assert_eq!(stages.current(), 0);
        assert_eq!(stages.total(), 3);
        assert!(!stages.is_complete());

        let s1 = stages.next("Stage 1");
        assert_eq!(stages.current(), 1);
        s1.finish_and_clear();

        let s2 = stages.next("Stage 2");
        assert_eq!(stages.current(), 2);
        s2.finish_and_clear();

        stages.skip("Stage 3");
        assert_eq!(stages.current(), 3);
        assert!(stages.is_complete());
    }

    #[test]
    fn test_empty_stage_progress() {
        let stages = StageProgress::new(0);
        assert!(stages.is_complete());
    }
}
