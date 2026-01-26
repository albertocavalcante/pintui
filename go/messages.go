// Package pintui provides a consistent terminal UI design system for Go.
//
// Messages module provides functions for printing different types of messages
// with consistent styling across your CLI application.
package pintui

import (
	"fmt"
	"os"

	"github.com/fatih/color"
)

var (
	infoIcon    = color.New(color.FgBlue).Sprint("ℹ")
	successIcon = color.New(color.FgGreen).Sprint("✓")
	warnIcon    = color.New(color.FgYellow).Sprint("⚠")
	errorIcon   = color.New(color.FgRed).Sprint("✗")
	dimStyle    = color.New(color.Faint)
)

// Info prints an info message with a blue ℹ icon.
//
// Use for general information that doesn't indicate success or failure.
//
// Example:
//
//	pintui.Info("Processing 42 files...")
//	// Output: ℹ Processing 42 files...
func Info(msg string) {
	fmt.Printf("%s %s\n", infoIcon, msg)
}

// Infof prints a formatted info message with a blue ℹ icon.
func Infof(format string, a ...any) {
	Info(fmt.Sprintf(format, a...))
}

// Success prints a success message with a green ✓ icon.
//
// Use when an operation completes successfully.
//
// Example:
//
//	pintui.Success("All tests passed")
//	// Output: ✓ All tests passed
func Success(msg string) {
	fmt.Printf("%s %s\n", successIcon, msg)
}

// Successf prints a formatted success message with a green ✓ icon.
func Successf(format string, a ...any) {
	Success(fmt.Sprintf(format, a...))
}

// Warn prints a warning message with a yellow ⚠ icon.
//
// Use for non-fatal issues that the user should be aware of.
//
// Example:
//
//	pintui.Warn("Config file not found, using defaults")
//	// Output: ⚠ Config file not found, using defaults
func Warn(msg string) {
	fmt.Printf("%s %s\n", warnIcon, msg)
}

// Warnf prints a formatted warning message with a yellow ⚠ icon.
func Warnf(format string, a ...any) {
	Warn(fmt.Sprintf(format, a...))
}

// Error prints an error message with a red ✗ icon to stderr.
//
// Use when an operation fails.
//
// Example:
//
//	pintui.Error("Failed to connect to database")
//	// Output (stderr): ✗ Failed to connect to database
func Error(msg string) {
	fmt.Fprintf(os.Stderr, "%s %s\n", errorIcon, msg)
}

// Errorf prints a formatted error message with a red ✗ icon to stderr.
func Errorf(format string, a ...any) {
	Error(fmt.Sprintf(format, a...))
}

// Dim prints a dim/muted message.
//
// Use for secondary information, details, or context that shouldn't
// draw attention away from primary content.
//
// Example:
//
//	pintui.Success("Build complete")
//	pintui.Dim("Output: ./bin/myapp")
//	pintui.Dim("Size: 4.2 MB")
func Dim(msg string) {
	fmt.Printf("  %s\n", dimStyle.Sprint(msg))
}

// Dimf prints a formatted dim/muted message.
func Dimf(format string, a ...any) {
	Dim(fmt.Sprintf(format, a...))
}
