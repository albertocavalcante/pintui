package pintui

import (
	"fmt"
	"os"
	"time"

	"github.com/briandowns/spinner"
	"github.com/fatih/color"
	"github.com/schollz/progressbar/v3"
)

// Spinner creates and starts a spinner for indeterminate progress.
//
// The spinner animates while work is in progress, showing the user
// that the application hasn't frozen.
//
// Example:
//
//	s := pintui.Spinner("Loading configuration...")
//	// ... do work ...
//	s.Success("Configuration loaded")
func Spinner(msg string) *SpinnerHandle {
	s := spinner.New(spinner.CharSets[14], 80*time.Millisecond) // CharSet 14 is ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏
	s.Suffix = " " + msg
	s.Color("cyan")
	s.Start()
	return &SpinnerHandle{spinner: s, msg: msg}
}

// SpinnerHandle wraps a spinner and provides finish methods.
type SpinnerHandle struct {
	spinner *spinner.Spinner
	msg     string
}

// Success stops the spinner and shows a success message.
//
// Example:
//
//	s := pintui.Spinner("Connecting...")
//	// ... do work ...
//	s.Success("Connected")
//	// Output: ✓ Connected
func (s *SpinnerHandle) Success(msg string) {
	s.spinner.Stop()
	fmt.Printf("\r%s %s\n", color.GreenString("✓"), msg)
}

// Error stops the spinner and shows an error message.
//
// Example:
//
//	s := pintui.Spinner("Connecting...")
//	// ... connection fails ...
//	s.Error("Connection failed")
//	// Output: ✗ Connection failed
func (s *SpinnerHandle) Error(msg string) {
	s.spinner.Stop()
	fmt.Fprintf(os.Stderr, "\r%s %s\n", color.RedString("✗"), msg)
}

// Warn stops the spinner and shows a warning message.
//
// Example:
//
//	s := pintui.Spinner("Checking...")
//	// ... partial success ...
//	s.Warn("Completed with warnings")
//	// Output: ⚠ Completed with warnings
func (s *SpinnerHandle) Warn(msg string) {
	s.spinner.Stop()
	fmt.Printf("\r%s %s\n", color.YellowString("⚠"), msg)
}

// Clear stops the spinner and clears it from display.
//
// Use when you want to remove the spinner without leaving a final message.
func (s *SpinnerHandle) Clear() {
	s.spinner.Stop()
	fmt.Print("\r\033[K") // Clear line
}

// UpdateMessage updates the spinner's message while it's running.
func (s *SpinnerHandle) UpdateMessage(msg string) {
	s.msg = msg
	s.spinner.Suffix = " " + msg
}

// Bar creates a progress bar for determinate progress.
//
// Use when you know the total amount of work to be done.
//
// Example:
//
//	bar := pintui.Bar(100, "Downloading")
//	for i := 0; i < 100; i++ {
//	    // ... do work ...
//	    bar.Add(1)
//	}
//	bar.Success("Downloaded")
func Bar(total int64, prefix string) *BarHandle {
	bar := progressbar.NewOptions64(total,
		progressbar.OptionSetDescription(prefix),
		progressbar.OptionSetWidth(40),
		progressbar.OptionEnableColorCodes(true),
		progressbar.OptionSetTheme(progressbar.Theme{
			Saucer:        "[cyan]━[reset]",
			SaucerHead:    "[cyan]╸[reset]",
			SaucerPadding: "[blue]─[reset]",
			BarStart:      "[",
			BarEnd:        "]",
		}),
		progressbar.OptionShowCount(),
		progressbar.OptionSpinnerType(14),
	)
	return &BarHandle{bar: bar}
}

// BarHandle wraps a progress bar and provides helper methods.
type BarHandle struct {
	bar *progressbar.ProgressBar
}

// Add increments the progress bar by the given amount.
func (b *BarHandle) Add(n int) {
	b.bar.Add(n)
}

// Add64 increments the progress bar by the given int64 amount.
func (b *BarHandle) Add64(n int64) {
	b.bar.Add64(n)
}

// Set sets the progress bar to a specific value.
func (b *BarHandle) Set(n int) {
	b.bar.Set(n)
}

// Set64 sets the progress bar to a specific int64 value.
func (b *BarHandle) Set64(n int64) {
	b.bar.Set64(n)
}

// Success finishes the progress bar with a success message.
func (b *BarHandle) Success(msg string) {
	b.bar.Finish()
	fmt.Printf("\r%s %s\n", color.GreenString("✓"), msg)
}

// Error finishes the progress bar with an error message.
func (b *BarHandle) Error(msg string) {
	b.bar.Finish()
	fmt.Fprintf(os.Stderr, "\r%s %s\n", color.RedString("✗"), msg)
}

// Clear finishes and clears the progress bar.
func (b *BarHandle) Clear() {
	b.bar.Clear()
}

// StageProgress tracks progress through a sequence of named stages.
//
// Example:
//
//	stages := pintui.NewStageProgress(3)
//
//	s1 := stages.Next("Downloading")
//	// ... download ...
//	s1.Success("Downloaded")
//
//	s2 := stages.Next("Extracting")
//	// ... extract ...
//	s2.Success("Extracted")
//
//	stages.Skip("Installing") // Skip if not needed
type StageProgress struct {
	current int
	total   int
}

// NewStageProgress creates a new stage progress tracker.
func NewStageProgress(total int) *StageProgress {
	return &StageProgress{current: 0, total: total}
}

// Next starts the next stage and returns a spinner for it.
//
// The spinner shows [current/total] before the stage name.
func (sp *StageProgress) Next(name string) *SpinnerHandle {
	sp.current++
	s := spinner.New(spinner.CharSets[14], 80*time.Millisecond)
	s.Prefix = fmt.Sprintf("[%d/%d] ", sp.current, sp.total)
	s.Suffix = " " + name
	s.Color("cyan")
	s.Start()
	return &SpinnerHandle{spinner: s, msg: name}
}

// Skip marks a stage as skipped.
func (sp *StageProgress) Skip(name string) {
	sp.current++
	fmt.Printf("  %s [%d/%d] %s (skipped)\n",
		color.New(color.Faint).Sprint("○"),
		sp.current, sp.total, name)
}

// Current returns the current stage number (1-indexed).
func (sp *StageProgress) Current() int {
	return sp.current
}

// Total returns the total number of stages.
func (sp *StageProgress) Total() int {
	return sp.total
}

// IsComplete returns true if all stages are complete.
func (sp *StageProgress) IsComplete() bool {
	return sp.current >= sp.total
}
