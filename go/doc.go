// Package pintui provides a consistent terminal UI design system for Go.
//
// Pintui is part of a cross-language design system that ensures your CLI tools
// have a consistent visual language regardless of implementation language.
//
// # Quick Start
//
//	package main
//
//	import "github.com/albertocavalcante/pintui/go"
//
//	func main() {
//	    // Messages
//	    pintui.Info("Starting operation...")
//	    pintui.Success("Done!")
//
//	    // Layout
//	    pintui.Header("Configuration")
//	    pintui.KV("Version", "1.0.0")
//
//	    // Progress
//	    spinner := pintui.Spinner("Loading...")
//	    // ... do work ...
//	    spinner.Success("Loaded")
//
//	    // Formatting
//	    size := pintui.HumanSize(1024 * 1024 * 50)
//	    pintui.KV("Size", size)  // "50.0 MB"
//	}
//
// # Design Philosophy
//
// Pintui provides opinionated defaults for a consistent look:
//   - Success: green checkmark (✓)
//   - Error: red cross (✗)
//   - Warning: yellow warning sign (⚠)
//   - Info: blue info icon (ℹ)
//   - Headers: cyan and bold
//
// This creates a unified visual language across all tools using pintui.
//
// # Modules
//
//   - Messages: Info, Success, Warn, Error, Dim
//   - Layout: Header, Section, KV, Step, Blank, Divider, Indent
//   - Progress: Spinner, Bar, StageProgress
//   - Format: HumanSize, ParseSize, TruncatePath, Pluralize, HumanDuration
//
// For more information, see https://github.com/albertocavalcante/pintui
package pintui
