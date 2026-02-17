//! Column-aligned tables and key-value groups for CLI output.
//!
//! This module provides simple column alignment for terminal output —
//! clean padding without ASCII-box decorations. Callers handle their
//! own coloring; the table just handles spacing.
//!
//! ## Table
//!
//! Auto-sized columns from content:
//!
//! ```no_run
//! use pintui::table::Table;
//!
//! let mut t = Table::new();
//! t.row(&["Package", "Status", "Size"]);
//! t.row(&["git", "installed", "3.2 MB"]);
//! t.row(&["neovim", "missing", ""]);
//! t.print();
//! ```
//!
//! Fixed column widths (0 = auto/remainder):
//!
//! ```no_run
//! use pintui::table::Table;
//!
//! let mut t = Table::aligned(&[20, 12, 0]);
//! t.row(&["Name", "Status", "Notes"]);
//! t.row(&["rust", "installed", "via rustup"]);
//! t.print();
//! ```
//!
//! ## Key-Value Group
//!
//! Aligned key-value pairs with consistent label width:
//!
//! ```no_run
//! use pintui::table::KvGroup;
//!
//! let mut kvs = KvGroup::new();
//! kvs.add("Local Cellar", "/opt/homebrew/Cellar");
//! kvs.add("External Cellar", "/Volumes/T9/homebrew/Cellar");
//! kvs.print();
//! ```

use colored::Colorize;
use unicode_width::UnicodeWidthStr;

// ---------------------------------------------------------------------------
// Table
// ---------------------------------------------------------------------------

/// A simple column-aligned table for CLI output.
///
/// Rows are collected first, then printed with consistent padding so
/// that columns line up. No box-drawing characters are used — just
/// clean spacing with a 2-space indent and 2-space column gap.
///
/// # Example
///
/// ```no_run
/// use pintui::table::Table;
///
/// let mut t = Table::new();
/// t.row(&["Name", "Version"]);
/// t.row(&["rustc", "1.85.0"]);
/// t.row(&["cargo", "1.85.0"]);
/// t.print();
/// // Output:
/// //   Name   Version
/// //   rustc  1.85.0
/// //   cargo  1.85.0
/// ```
pub struct Table {
    /// Pre-configured column widths (empty means auto-size everything).
    fixed_widths: Vec<usize>,
    /// Accumulated rows of cell strings.
    rows: Vec<Vec<String>>,
}

impl Table {
    /// Create a new table with automatic column sizing.
    ///
    /// Column widths are determined by the widest cell in each column
    /// across all rows.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            fixed_widths: Vec::new(),
            rows: Vec::new(),
        }
    }

    /// Create a new table with fixed column widths.
    ///
    /// A width of `0` means the column auto-sizes to its content
    /// (same as the default behaviour for that column).
    ///
    /// # Arguments
    ///
    /// * `widths` - Desired display-width for each column. Use `0` for
    ///   auto-sizing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pintui::table::Table;
    ///
    /// let mut t = Table::aligned(&[20, 12, 0]);
    /// t.row(&["Package", "Status", "Notes"]);
    /// t.print();
    /// ```
    #[must_use]
    pub fn aligned(widths: &[usize]) -> Self {
        Self {
            fixed_widths: widths.to_vec(),
            rows: Vec::new(),
        }
    }

    /// Add a row of cells.
    ///
    /// Each element of `cells` becomes one column value. Rows may have
    /// different numbers of columns — shorter rows are treated as if
    /// the missing columns were empty strings.
    pub fn row(&mut self, cells: &[&str]) {
        self.rows.push(cells.iter().map(|s| (*s).to_string()).collect());
    }

    /// Print all rows with consistent column alignment.
    ///
    /// Each row is indented by 2 spaces, and columns are separated by
    /// 2 spaces. The last column in each row is printed without trailing
    /// padding.
    ///
    /// If the table has no rows, nothing is printed.
    pub fn print(&self) {
        if self.rows.is_empty() {
            return;
        }

        let widths = self.resolve_widths();

        for row in &self.rows {
            let mut line = String::from("  ");
            let col_count = widths.len();

            for (i, width) in widths.iter().enumerate() {
                let cell = row.get(i).map_or("", String::as_str);
                line.push_str(cell);

                // Last column: no trailing padding.
                if i < col_count - 1 {
                    let cell_width = UnicodeWidthStr::width(cell);
                    let pad = width.saturating_sub(cell_width);
                    for _ in 0..pad {
                        line.push(' ');
                    }
                    // Column gap.
                    line.push_str("  ");
                }
            }

            println!("{}", line.trim_end());
        }
    }

    /// Compute the effective width for every column.
    fn resolve_widths(&self) -> Vec<usize> {
        let num_cols = self.max_columns();
        let mut widths = vec![0_usize; num_cols];

        // Measure the widest cell per column.
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                let w = UnicodeWidthStr::width(cell.as_str());
                if w > widths[i] {
                    widths[i] = w;
                }
            }
        }

        // Override with fixed widths where specified and non-zero.
        for (i, &fixed) in self.fixed_widths.iter().enumerate() {
            if i < num_cols && fixed > 0 {
                widths[i] = fixed;
            }
        }

        widths
    }

    /// Determine the total number of columns across all rows.
    fn max_columns(&self) -> usize {
        self.rows.iter().map(Vec::len).max().unwrap_or(0)
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// KvGroup
// ---------------------------------------------------------------------------

/// A group of key-value pairs printed with aligned keys.
///
/// All pairs are collected first so the longest key can be measured.
/// Then every row is printed with the key right-padded to that width,
/// followed by a colon and the value.
///
/// Keys are dimmed (matching the existing [`crate::layout::kv`]
/// convention) and the whole block has a 2-space indent.
///
/// # Example
///
/// ```no_run
/// use pintui::table::KvGroup;
///
/// let mut kvs = KvGroup::new();
/// kvs.add("Local Cellar", "/opt/homebrew/Cellar");
/// kvs.add("External Cellar", "/Volumes/T9/homebrew/Cellar");
/// kvs.print();
/// // Output:
/// //   Local Cellar   : /opt/homebrew/Cellar
/// //   External Cellar: /Volumes/T9/homebrew/Cellar
/// ```
pub struct KvGroup {
    pairs: Vec<(String, String)>,
}

impl KvGroup {
    /// Create a new, empty key-value group.
    #[must_use]
    pub const fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    /// Add a key-value pair to the group.
    pub fn add(&mut self, key: &str, value: &str) {
        self.pairs.push((key.to_string(), value.to_string()));
    }

    /// Print all pairs with aligned keys.
    ///
    /// If the group is empty, nothing is printed.
    pub fn print(&self) {
        if self.pairs.is_empty() {
            return;
        }

        let max_w = self.max_key_width();

        for (key, value) in &self.pairs {
            let key_width = UnicodeWidthStr::width(key.as_str());
            let pad = max_w.saturating_sub(key_width);
            let padded_key = format!("{key}{}", " ".repeat(pad));
            println!("  {}: {value}", padded_key.dimmed());
        }
    }

    /// Compute the display width of the longest key.
    ///
    /// Returns 0 if the group is empty. Uses Unicode display width
    /// for correct measurement of CJK and other wide characters.
    fn max_key_width(&self) -> usize {
        self.pairs
            .iter()
            .map(|(k, _)| UnicodeWidthStr::width(k.as_str()))
            .max()
            .unwrap_or(0)
    }
}

impl Default for KvGroup {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // =====================================================================
    // Table — column counts
    // =====================================================================

    #[test]
    fn table_zero_columns() {
        let mut t = Table::new();
        t.row(&[]);
        t.print(); // should not panic
    }

    #[test]
    fn table_one_column() {
        let mut t = Table::new();
        t.row(&["alpha"]);
        t.row(&["beta"]);
        t.print();
    }

    #[test]
    fn table_two_columns() {
        let mut t = Table::new();
        t.row(&["Name", "Value"]);
        t.row(&["foo", "123"]);
        t.print();
    }

    #[test]
    fn table_three_columns() {
        let mut t = Table::new();
        t.row(&["Package", "Status", "Size"]);
        t.row(&["git", "installed", "3.2 MB"]);
        t.row(&["neovim", "missing", ""]);
        t.print();
    }

    // =====================================================================
    // Table — empty
    // =====================================================================

    #[test]
    fn table_empty_does_not_panic() {
        let t = Table::new();
        t.print(); // zero rows — prints nothing
    }

    // =====================================================================
    // Table — unicode content
    // =====================================================================

    #[test]
    fn table_unicode_content() {
        let mut t = Table::new();
        t.row(&["名前", "状態"]);
        t.row(&["ギット", "インストール済み"]);
        t.row(&["vim", "未導入"]);
        t.print();
    }

    #[test]
    fn table_mixed_ascii_and_cjk() {
        let mut t = Table::new();
        t.row(&["Tool", "説明"]);
        t.row(&["rustc", "コンパイラ"]);
        t.row(&["cargo", "build tool"]);
        t.print();
    }

    // =====================================================================
    // Table — fixed widths
    // =====================================================================

    #[test]
    fn table_aligned_fixed_widths() {
        let mut t = Table::aligned(&[20, 10, 0]);
        t.row(&["Package", "Status", "Notes"]);
        t.row(&["rust", "installed", "via rustup"]);
        t.print();
    }

    #[test]
    fn table_aligned_zero_means_auto() {
        let mut t = Table::aligned(&[0, 0]);
        t.row(&["a", "b"]);
        t.row(&["longer", "x"]);
        t.print();
    }

    // =====================================================================
    // Table — ragged rows (different column counts)
    // =====================================================================

    #[test]
    fn table_ragged_rows() {
        let mut t = Table::new();
        t.row(&["a", "b", "c"]);
        t.row(&["x"]);
        t.row(&["1", "2"]);
        t.print();
    }

    // =====================================================================
    // Table — resolve_widths correctness
    // =====================================================================

    #[test]
    fn table_resolve_widths_auto() {
        let mut t = Table::new();
        t.row(&["ab", "cdef"]);
        t.row(&["ghijk", "l"]);
        let widths = t.resolve_widths();
        assert_eq!(widths, vec![5, 4]);
    }

    #[test]
    fn table_resolve_widths_fixed_override() {
        let mut t = Table::aligned(&[10, 0]);
        t.row(&["short", "x"]);
        let widths = t.resolve_widths();
        assert_eq!(widths[0], 10);
        assert_eq!(widths[1], 1);
    }

    #[test]
    fn table_resolve_widths_unicode() {
        let mut t = Table::new();
        // CJK characters are 2 display-columns wide each.
        t.row(&["名前", "abc"]);
        let widths = t.resolve_widths();
        assert_eq!(widths[0], 4); // 2 chars * 2 columns each
        assert_eq!(widths[1], 3);
    }

    // =====================================================================
    // KvGroup
    // =====================================================================

    #[test]
    fn kvgroup_empty_does_not_panic() {
        let kvs = KvGroup::new();
        kvs.print();
    }

    #[test]
    fn kvgroup_single_pair() {
        let mut kvs = KvGroup::new();
        kvs.add("Key", "Value");
        kvs.print();
    }

    #[test]
    fn kvgroup_varying_key_lengths() {
        let mut kvs = KvGroup::new();
        kvs.add("A", "short key");
        kvs.add("Medium Key", "medium");
        kvs.add("A Very Long Key Name", "long");
        kvs.print();
    }

    #[test]
    fn kvgroup_unicode_keys() {
        let mut kvs = KvGroup::new();
        kvs.add("名前", "Rust");
        kvs.add("バージョン", "1.85");
        kvs.add("OS", "macOS");
        kvs.print();
    }

    // =====================================================================
    // KvGroup — max_key_width correctness
    // =====================================================================

    #[test]
    fn kvgroup_max_key_width_empty() {
        let kvs = KvGroup::new();
        assert_eq!(kvs.max_key_width(), 0);
    }

    #[test]
    fn kvgroup_max_key_width_ascii() {
        let mut kvs = KvGroup::new();
        kvs.add("A", "1");
        kvs.add("Medium Key", "2");
        kvs.add("A Very Long Key Name", "3");
        // "A Very Long Key Name" = 20 ASCII chars = 20 display columns
        assert_eq!(kvs.max_key_width(), 20);
    }

    #[test]
    fn kvgroup_max_key_width_cjk() {
        let mut kvs = KvGroup::new();
        kvs.add("名前", "Rust");           // 2 CJK * 2 = 4
        kvs.add("バージョン", "1.85");     // 5 CJK * 2 = 10
        kvs.add("OS", "macOS");             // 2 ASCII = 2
        // Longest is "バージョン" = 10 display columns
        assert_eq!(kvs.max_key_width(), 10);
    }

    #[test]
    fn kvgroup_max_key_width_mixed() {
        let mut kvs = KvGroup::new();
        kvs.add("Local Cellar", "a");       // 12
        kvs.add("External日本", "b");       // 8 ASCII + 2 CJK*2 = 12
        assert_eq!(kvs.max_key_width(), 12);
    }

    // =====================================================================
    // Default impls
    // =====================================================================

    #[test]
    fn table_default() {
        let t = Table::default();
        t.print();
    }

    #[test]
    fn kvgroup_default() {
        let kvs = KvGroup::default();
        kvs.print();
    }
}
