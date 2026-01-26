package pintui

import (
	"fmt"
	"strconv"
	"strings"
	"time"
)

const (
	KB uint64 = 1024
	MB uint64 = KB * 1024
	GB uint64 = MB * 1024
	TB uint64 = GB * 1024
)

// HumanSize formats bytes as a human-readable size string.
//
// Automatically selects the appropriate unit (B, KB, MB, GB, TB)
// based on the magnitude of the value.
//
// Example:
//
//	pintui.HumanSize(0)                    // "0 B"
//	pintui.HumanSize(1023)                 // "1023 B"
//	pintui.HumanSize(1024)                 // "1.0 KB"
//	pintui.HumanSize(1024 * 1024)          // "1.0 MB"
//	pintui.HumanSize(1024 * 1024 * 1024)   // "1.0 GB"
func HumanSize(bytes uint64) string {
	switch {
	case bytes >= TB:
		return fmt.Sprintf("%.2f TB", float64(bytes)/float64(TB))
	case bytes >= GB:
		return fmt.Sprintf("%.1f GB", float64(bytes)/float64(GB))
	case bytes >= MB:
		return fmt.Sprintf("%.1f MB", float64(bytes)/float64(MB))
	case bytes >= KB:
		return fmt.Sprintf("%.1f KB", float64(bytes)/float64(KB))
	default:
		return fmt.Sprintf("%d B", bytes)
	}
}

// ParseSize parses a human-readable size string into bytes.
//
// Supports suffixes: B, KB, MB, GB, TB (case-insensitive).
// Numbers without suffixes are treated as bytes.
// Decimal values are supported (e.g., "1.5GB").
//
// Example:
//
//	pintui.ParseSize("100")      // 100, nil
//	pintui.ParseSize("1KB")      // 1024, nil
//	pintui.ParseSize("100MB")    // 104857600, nil
//	pintui.ParseSize("1.5GB")    // 1610612736, nil
//	pintui.ParseSize("invalid")  // 0, error
func ParseSize(s string) (uint64, error) {
	s = strings.TrimSpace(strings.ToUpper(s))

	if s == "" {
		return 0, fmt.Errorf("empty size string")
	}

	var numStr string
	var multiplier uint64 = 1

	switch {
	case strings.HasSuffix(s, "TB"):
		numStr = strings.TrimSuffix(s, "TB")
		multiplier = TB
	case strings.HasSuffix(s, "GB"):
		numStr = strings.TrimSuffix(s, "GB")
		multiplier = GB
	case strings.HasSuffix(s, "MB"):
		numStr = strings.TrimSuffix(s, "MB")
		multiplier = MB
	case strings.HasSuffix(s, "KB"):
		numStr = strings.TrimSuffix(s, "KB")
		multiplier = KB
	case strings.HasSuffix(s, "B"):
		numStr = strings.TrimSuffix(s, "B")
	default:
		numStr = s
	}

	numStr = strings.TrimSpace(numStr)
	num, err := strconv.ParseFloat(numStr, 64)
	if err != nil {
		return 0, fmt.Errorf("invalid number in size: '%s'", numStr)
	}

	if num < 0 {
		return 0, fmt.Errorf("size cannot be negative: %f", num)
	}

	return uint64(num * float64(multiplier)), nil
}

// TruncatePath truncates a path string for display, preserving the end.
//
// When a path is longer than maxLen, it's truncated from the
// beginning and prefixed with "..." to fit within the limit.
//
// This preserves the filename and immediate parent directories,
// which are usually the most relevant parts of a path.
//
// Example:
//
//	pintui.TruncatePath("short.txt", 20)                    // "short.txt"
//	pintui.TruncatePath("/very/long/path/to/file.txt", 15) // ".../to/file.txt"
//	pintui.TruncatePath("test", 3)                          // "..."
func TruncatePath(path string, maxLen int) string {
	if len(path) <= maxLen {
		return path
	}
	if maxLen <= 3 {
		return "..."
	}
	return "..." + path[len(path)-maxLen+3:]
}

// Pluralize formats a count with the appropriate singular/plural form.
//
// Example:
//
//	pintui.Pluralize(0, "file", "files")  // "0 files"
//	pintui.Pluralize(1, "file", "files")  // "1 file"
//	pintui.Pluralize(5, "file", "files")  // "5 files"
func Pluralize(count int, singular, plural string) string {
	if count == 1 {
		return fmt.Sprintf("%d %s", count, singular)
	}
	return fmt.Sprintf("%d %s", count, plural)
}

// HumanDuration formats a duration in a human-readable way.
//
// Example:
//
//	pintui.HumanDuration(500 * time.Millisecond)  // "500ms"
//	pintui.HumanDuration(5 * time.Second)         // "5.0s"
//	pintui.HumanDuration(90 * time.Second)        // "1m 30s"
//	pintui.HumanDuration(3661 * time.Second)      // "1h 1m"
func HumanDuration(d time.Duration) string {
	secs := int64(d.Seconds())
	millis := d.Milliseconds() % 1000

	switch {
	case secs == 0:
		return fmt.Sprintf("%dms", d.Milliseconds())
	case secs < 60:
		return fmt.Sprintf("%d.%ds", secs, millis/100)
	case secs < 3600:
		mins := secs / 60
		remainingSecs := secs % 60
		return fmt.Sprintf("%dm %ds", mins, remainingSecs)
	default:
		hours := secs / 3600
		remainingMins := (secs % 3600) / 60
		return fmt.Sprintf("%dh %dm", hours, remainingMins)
	}
}
