//! Formatting utilities for human-readable output.
//!
//! This module provides functions for converting data into
//! human-friendly string representations.
//!
//! ## Size Formatting
//!
//! ```
//! use pintui::format;
//!
//! assert_eq!(format::human_size(1024), "1.0 KB");
//! assert_eq!(format::human_size(1024 * 1024 * 50), "50.0 MB");
//! assert_eq!(format::human_size(1024u64.pow(4)), "1.00 TB");
//! ```
//!
//! ## Size Parsing
//!
//! ```
//! use pintui::format;
//!
//! assert_eq!(format::parse_size("100MB").unwrap(), 104_857_600);
//! assert_eq!(format::parse_size("1.5GB").unwrap(), 1_610_612_736);
//! ```

/// Byte size constants for formatting.
const KB: u64 = 1024;
const MB: u64 = KB * 1024;
const GB: u64 = MB * 1024;
const TB: u64 = GB * 1024;

/// Format bytes as a human-readable size string.
///
/// Automatically selects the appropriate unit (B, KB, MB, GB, TB)
/// based on the magnitude of the value.
///
/// # Examples
///
/// ```
/// use pintui::format::human_size;
///
/// assert_eq!(human_size(0), "0 B");
/// assert_eq!(human_size(1023), "1023 B");
/// assert_eq!(human_size(1024), "1.0 KB");
/// assert_eq!(human_size(1024 * 1024), "1.0 MB");
/// assert_eq!(human_size(1024 * 1024 * 1024), "1.0 GB");
/// assert_eq!(human_size(1024u64 * 1024 * 1024 * 1024), "1.00 TB");
/// ```
pub fn human_size(bytes: u64) -> String {
    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

/// Parse a human-readable size string into bytes.
///
/// Supports suffixes: B, KB, MB, GB, TB (case-insensitive).
/// Numbers without suffixes are treated as bytes.
/// Decimal values are supported (e.g., "1.5GB").
///
/// # Examples
///
/// ```
/// use pintui::format::parse_size;
///
/// // Without suffix (bytes)
/// assert_eq!(parse_size("100").unwrap(), 100);
///
/// // With suffix
/// assert_eq!(parse_size("1KB").unwrap(), 1024);
/// assert_eq!(parse_size("100MB").unwrap(), 104_857_600);
/// assert_eq!(parse_size("1.5GB").unwrap(), 1_610_612_736);
///
/// // Case insensitive
/// assert_eq!(parse_size("1kb").unwrap(), 1024);
/// assert_eq!(parse_size("1Kb").unwrap(), 1024);
///
/// // Whitespace is trimmed
/// assert_eq!(parse_size("  100MB  ").unwrap(), 104_857_600);
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The string is empty
/// - The numeric part cannot be parsed
/// - The value is negative
pub fn parse_size(size_str: &str) -> Result<u64, String> {
    let size_str = size_str.trim().to_uppercase();

    if size_str.is_empty() {
        return Err("Empty size string".to_string());
    }

    let (num_str, multiplier) = if let Some(num) = size_str.strip_suffix("TB") {
        (num, TB)
    } else if let Some(num) = size_str.strip_suffix("GB") {
        (num, GB)
    } else if let Some(num) = size_str.strip_suffix("MB") {
        (num, MB)
    } else if let Some(num) = size_str.strip_suffix("KB") {
        (num, KB)
    } else if let Some(num) = size_str.strip_suffix('B') {
        (num, 1u64)
    } else {
        // Assume bytes if no suffix
        (size_str.as_str(), 1u64)
    };

    let num: f64 = num_str
        .trim()
        .parse()
        .map_err(|_| format!("Invalid number in size: '{}'", num_str.trim()))?;

    if num < 0.0 {
        return Err(format!("Size cannot be negative: {num}"));
    }

    Ok((num * multiplier as f64) as u64)
}

/// Truncate a path string for display, preserving the end.
///
/// When a path is longer than `max_len`, it's truncated from the
/// beginning and prefixed with "..." to fit within the limit.
///
/// This preserves the filename and immediate parent directories,
/// which are usually the most relevant parts of a path.
///
/// # Examples
///
/// ```
/// use pintui::format::truncate_path;
///
/// // Short paths are unchanged
/// assert_eq!(truncate_path("short.txt", 20), "short.txt");
///
/// // Long paths are truncated from the start
/// assert_eq!(
///     truncate_path("/very/long/path/to/file.txt", 15),
///     ".../to/file.txt"
/// );
///
/// // Edge cases
/// assert_eq!(truncate_path("test", 3), "...");
/// assert_eq!(truncate_path("", 10), "");
/// ```
pub fn truncate_path(path: &str, max_len: usize) -> String {
    // Fast path: if byte length fits, char count certainly fits too
    // (every char is at least 1 byte).
    if path.len() <= max_len {
        return path.to_string();
    }

    let char_count = path.chars().count();
    if char_count <= max_len {
        return path.to_string();
    }

    if max_len <= 3 {
        return "...".to_string();
    }

    let skip = char_count - (max_len - 3);
    let byte_offset = path
        .char_indices()
        .nth(skip)
        .map_or(path.len(), |(i, _)| i);
    format!("...{}", &path[byte_offset..])
}

/// Format a count with singular/plural form.
///
/// # Examples
///
/// ```
/// use pintui::format::pluralize;
///
/// assert_eq!(pluralize(0, "file", "files"), "0 files");
/// assert_eq!(pluralize(1, "file", "files"), "1 file");
/// assert_eq!(pluralize(5, "file", "files"), "5 files");
/// ```
pub fn pluralize(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("{count} {singular}")
    } else {
        format!("{count} {plural}")
    }
}

/// Format a number with comma separators for readability.
///
/// Inserts commas every three digits from the right for numbers
/// 1,000 and above. Numbers below 1,000 are returned unchanged.
///
/// # Examples
///
/// ```
/// use pintui::format::human_count;
///
/// assert_eq!(human_count(0), "0");
/// assert_eq!(human_count(42), "42");
/// assert_eq!(human_count(999), "999");
/// assert_eq!(human_count(1_000), "1,000");
/// assert_eq!(human_count(1_234_567), "1,234,567");
/// assert_eq!(human_count(1_000_000_000), "1,000,000,000");
/// ```
pub fn human_count(n: u64) -> String {
    let s = n.to_string();
    if s.len() <= 3 {
        return s;
    }

    let mut result = String::with_capacity(s.len() + s.len() / 3);
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result
}

/// Format a duration in a human-readable way.
///
/// # Examples
///
/// ```
/// use pintui::format::human_duration;
/// use std::time::Duration;
///
/// assert_eq!(human_duration(Duration::from_millis(500)), "500ms");
/// assert_eq!(human_duration(Duration::from_secs(5)), "5.0s");
/// assert_eq!(human_duration(Duration::from_secs(90)), "1m 30s");
/// assert_eq!(human_duration(Duration::from_secs(3661)), "1h 1m");
/// ```
pub fn human_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();

    if secs == 0 {
        format!("{millis}ms")
    } else if secs < 60 {
        format!("{secs}.{}s", millis / 100)
    } else if secs < 3600 {
        let mins = secs / 60;
        let remaining_secs = secs % 60;
        format!("{mins}m {remaining_secs}s")
    } else {
        let hours = secs / 3600;
        let remaining_mins = (secs % 3600) / 60;
        format!("{hours}h {remaining_mins}m")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // =========================================================================
    // human_size tests
    // =========================================================================

    #[test]
    fn test_human_size_bytes() {
        assert_eq!(human_size(0), "0 B");
        assert_eq!(human_size(100), "100 B");
        assert_eq!(human_size(1023), "1023 B");
    }

    #[test]
    fn test_human_size_kb() {
        assert_eq!(human_size(1024), "1.0 KB");
        assert_eq!(human_size(1536), "1.5 KB");
        assert_eq!(human_size(10240), "10.0 KB");
    }

    #[test]
    fn test_human_size_mb() {
        assert_eq!(human_size(1024 * 1024), "1.0 MB");
        assert_eq!(human_size(1024 * 1024 * 100), "100.0 MB");
    }

    #[test]
    fn test_human_size_gb() {
        assert_eq!(human_size(1024 * 1024 * 1024), "1.0 GB");
        assert_eq!(
            human_size(1024 * 1024 * 1024 * 2 + 1024 * 1024 * 512),
            "2.5 GB"
        );
    }

    #[test]
    fn test_human_size_tb() {
        assert_eq!(human_size(1024u64 * 1024 * 1024 * 1024), "1.00 TB");
        assert_eq!(human_size(1024u64 * 1024 * 1024 * 1024 * 2), "2.00 TB");
    }

    // =========================================================================
    // parse_size tests
    // =========================================================================

    #[test]
    fn test_parse_size_bytes() {
        assert_eq!(parse_size("100").unwrap(), 100);
        assert_eq!(parse_size("100B").unwrap(), 100);
        assert_eq!(parse_size("100b").unwrap(), 100);
    }

    #[test]
    fn test_parse_size_kb() {
        assert_eq!(parse_size("1KB").unwrap(), 1024);
        assert_eq!(parse_size("1kb").unwrap(), 1024);
        assert_eq!(parse_size("10KB").unwrap(), 10240);
    }

    #[test]
    fn test_parse_size_mb() {
        assert_eq!(parse_size("1MB").unwrap(), 1024 * 1024);
        assert_eq!(parse_size("100MB").unwrap(), 100 * 1024 * 1024);
        assert_eq!(parse_size("1.5MB").unwrap(), (1.5 * 1024.0 * 1024.0) as u64);
    }

    #[test]
    fn test_parse_size_gb() {
        assert_eq!(parse_size("1GB").unwrap(), 1024 * 1024 * 1024);
        assert_eq!(parse_size("2GB").unwrap(), 2 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_parse_size_tb() {
        assert_eq!(parse_size("1TB").unwrap(), 1024u64 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_parse_size_whitespace() {
        assert_eq!(parse_size("  100MB  ").unwrap(), 100 * 1024 * 1024);
        assert_eq!(parse_size(" 1 GB").unwrap(), 1024 * 1024 * 1024);
    }

    #[test]
    fn test_parse_size_errors() {
        assert!(parse_size("").is_err());
        assert!(parse_size("abc").is_err());
        assert!(parse_size("MB").is_err());
        assert!(parse_size("-100MB").is_err());
    }

    // =========================================================================
    // truncate_path tests
    // =========================================================================

    #[test]
    fn test_truncate_path_short() {
        assert_eq!(truncate_path("short.txt", 20), "short.txt");
        assert_eq!(truncate_path("exact", 5), "exact");
    }

    #[test]
    fn test_truncate_path_long() {
        assert_eq!(
            truncate_path("/very/long/path/to/file.txt", 15),
            ".../to/file.txt"
        );
    }

    #[test]
    fn test_truncate_path_edge_cases() {
        assert_eq!(truncate_path("test", 3), "...");
        assert_eq!(truncate_path("test", 2), "...");
        assert_eq!(truncate_path("", 10), "");
    }

    #[test]
    fn test_truncate_path_multibyte() {
        // "test日" is 7 bytes but 5 chars — fits in max_len=5.
        assert_eq!(truncate_path("test日", 5), "test日");

        // "test日本語" is 13 bytes, 7 chars. max_len=5 → keep 2 chars from end.
        assert_eq!(truncate_path("test日本語", 5), "...本語");

        // Multi-byte at the truncation boundary.
        assert_eq!(truncate_path("日本語test", 6), "...est");

        // All multi-byte, 5 chars fits in max_len=5.
        assert_eq!(truncate_path("日本語漢字", 5), "日本語漢字");

        // All multi-byte, 5 chars exceeds max_len=4.
        assert_eq!(truncate_path("日本語漢字", 4), "...字");
    }

    // =========================================================================
    // pluralize tests
    // =========================================================================

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize(0, "file", "files"), "0 files");
        assert_eq!(pluralize(1, "file", "files"), "1 file");
        assert_eq!(pluralize(2, "file", "files"), "2 files");
        assert_eq!(pluralize(100, "item", "items"), "100 items");
    }

    // =========================================================================
    // human_duration tests
    // =========================================================================

    #[test]
    fn test_human_duration_millis() {
        assert_eq!(human_duration(Duration::from_millis(0)), "0ms");
        assert_eq!(human_duration(Duration::from_millis(500)), "500ms");
        assert_eq!(human_duration(Duration::from_millis(999)), "999ms");
    }

    #[test]
    fn test_human_duration_seconds() {
        assert_eq!(human_duration(Duration::from_secs(1)), "1.0s");
        assert_eq!(human_duration(Duration::from_secs(5)), "5.0s");
        assert_eq!(human_duration(Duration::from_millis(5500)), "5.5s");
    }

    #[test]
    fn test_human_duration_minutes() {
        assert_eq!(human_duration(Duration::from_secs(60)), "1m 0s");
        assert_eq!(human_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(human_duration(Duration::from_secs(3599)), "59m 59s");
    }

    #[test]
    fn test_human_duration_hours() {
        assert_eq!(human_duration(Duration::from_secs(3600)), "1h 0m");
        assert_eq!(human_duration(Duration::from_secs(3661)), "1h 1m");
        assert_eq!(human_duration(Duration::from_secs(7200)), "2h 0m");
    }

    // =========================================================================
    // human_count tests
    // =========================================================================

    #[test]
    fn test_human_count_small() {
        assert_eq!(human_count(0), "0");
        assert_eq!(human_count(1), "1");
        assert_eq!(human_count(42), "42");
        assert_eq!(human_count(999), "999");
    }

    #[test]
    fn test_human_count_thousands() {
        assert_eq!(human_count(1_000), "1,000");
        assert_eq!(human_count(1_234), "1,234");
        assert_eq!(human_count(10_000), "10,000");
        assert_eq!(human_count(999_999), "999,999");
    }

    #[test]
    fn test_human_count_millions() {
        assert_eq!(human_count(1_000_000), "1,000,000");
        assert_eq!(human_count(1_234_567), "1,234,567");
    }

    #[test]
    fn test_human_count_billions() {
        assert_eq!(human_count(1_000_000_000), "1,000,000,000");
    }
}
