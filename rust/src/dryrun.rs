//! Dry-run output helpers for previewing actions without side effects.
//!
//! When a CLI tool runs in dry-run mode it should show the user what
//! *would* happen without actually performing any changes. This module
//! provides two small helpers for that purpose:
//!
//! - [`action`] — prints a single planned action line with an arrow prefix.
//! - [`footer`] — prints a closing notice that no changes were made.
//!
//! ## Example
//!
//! ```no_run
//! use pintui::dryrun;
//!
//! dryrun::action("Would trim", "git (4.2 MB)");
//! dryrun::action("Would trim", "node_modules (120 MB)");
//! dryrun::footer();
//! // Output:
//! //   → Would trim git (4.2 MB)
//! //   → Would trim node_modules (120 MB)
//! // ⚠ Dry run — no changes made
//! ```

use crate::icons;

/// Print a dry-run action line with a 2-space indent and a cyan arrow prefix.
///
/// The output format is `  → {verb} {detail}`, matching the standard
/// checklist indent used throughout pintui.
///
/// # Example
///
/// ```no_run
/// pintui::dryrun::action("Would remove", "stale-cache (8.1 MB)");
/// // Output:   → Would remove stale-cache (8.1 MB)
/// ```
pub fn action(verb: &str, detail: &str) {
    println!("  {} {verb} {detail}", icons::arrow());
}

/// Print a dry-run footer indicating that no changes were made.
///
/// Outputs `⚠ Dry run — no changes made` using the yellow warning icon.
/// Call this once at the end of a dry-run session.
///
/// # Example
///
/// ```no_run
/// pintui::dryrun::footer();
/// // Output: ⚠ Dry run — no changes made
/// ```
pub fn footer() {
    println!("{} Dry run \u{2014} no changes made", icons::warn());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_does_not_panic() {
        action("Would trim", "git (4.2 MB)");
    }

    #[test]
    fn test_footer_does_not_panic() {
        footer();
    }

    #[test]
    fn test_action_empty_strings() {
        action("", "");
    }

    #[test]
    fn test_action_unicode() {
        action("Would delete", "キャッシュ (1.3 GB)");
    }

    #[test]
    fn test_multiple_actions_then_footer() {
        action("Would trim", "git (4.2 MB)");
        action("Would trim", "node_modules (120 MB)");
        footer();
    }
}
