package pintui

import (
	"fmt"
	"strings"

	"github.com/fatih/color"
)

var (
	headerStyle  = color.New(color.Bold)
	sectionStyle = color.New(color.FgCyan, color.Bold)
	keyStyle     = color.New(color.Faint)
	stepStyle    = color.New(color.FgBlue, color.Bold)
	dividerStyle = color.New(color.Faint)
)

// Header prints a header/title with an underline.
//
// Headers are bold and followed by a dimmed line of the same width.
// A blank line is printed before the header for visual separation.
//
// Example:
//
//	pintui.Header("Configuration")
//	// Output:
//	//
//	// Configuration
//	// ─────────────
func Header(title string) {
	fmt.Println()
	headerStyle.Println(title)
	dividerStyle.Println(strings.Repeat("─", len(title)))
}

// Section prints a section header.
//
// Sections are cyan and bold, with a blank line before for separation.
// Use sections to group related information under a header.
//
// Example:
//
//	pintui.Section("Dependencies")
//	// Output:
//	//
//	// Dependencies
func Section(title string) {
	fmt.Println()
	sectionStyle.Println(title)
}

// KV prints a key-value pair.
//
// The key is dimmed and the value is normal. Both are indented.
// Use for displaying configuration, status, or metadata.
//
// Example:
//
//	pintui.KV("Version", "1.0.0")
//	pintui.KV("Status", "Active")
//	// Output:
//	//   Version: 1.0.0
//	//   Status: Active
func KV(key, value string) {
	fmt.Printf("  %s: %s\n", keyStyle.Sprint(key), value)
}

// KVf prints a key-value pair with a formatted value.
func KVf(key, format string, a ...any) {
	KV(key, fmt.Sprintf(format, a...))
}

// Step prints a step indicator for multi-step operations.
//
// Shows progress through a sequence of steps with [current/total] prefix.
// The indicator is blue and bold for visibility.
//
// Example:
//
//	pintui.Step(1, 3, "Fetching dependencies")
//	pintui.Step(2, 3, "Compiling")
//	pintui.Step(3, 3, "Linking")
//	// Output:
//	// [1/3] Fetching dependencies
//	// [2/3] Compiling
//	// [3/3] Linking
func Step(num, total int, msg string) {
	fmt.Printf("%s %s\n", stepStyle.Sprintf("[%d/%d]", num, total), msg)
}

// Stepf prints a step indicator with a formatted message.
func Stepf(num, total int, format string, a ...any) {
	Step(num, total, fmt.Sprintf(format, a...))
}

// Blank prints a blank line for visual separation.
//
// Convenience function for adding vertical whitespace.
func Blank() {
	fmt.Println()
}

// Divider prints a horizontal rule/divider.
//
// Example:
//
//	pintui.Divider(40)
//	// Output:
//	// ────────────────────────────────────────
func Divider(width int) {
	dividerStyle.Println(strings.Repeat("─", width))
}

// Indent prints an indented line.
//
// Each level adds 2 spaces of indentation.
//
// Example:
//
//	pintui.Indent(1, "First level")
//	pintui.Indent(2, "Second level")
//	// Output:
//	//   First level
//	//     Second level
func Indent(level int, msg string) {
	fmt.Printf("%s%s\n", strings.Repeat("  ", level), msg)
}

// Indentf prints an indented line with a formatted message.
func Indentf(level int, format string, a ...any) {
	Indent(level, fmt.Sprintf(format, a...))
}
