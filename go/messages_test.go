package pintui

import "testing"

// TestMessagesDoNotPanic verifies that message functions don't panic
func TestMessagesDoNotPanic(t *testing.T) {
	// These functions print to stdout/stderr, so we mainly test
	// that they don't panic

	t.Run("Info", func(t *testing.T) {
		Info("test message")
		Infof("formatted %s", "message")
	})

	t.Run("Success", func(t *testing.T) {
		Success("test message")
		Successf("formatted %s", "message")
	})

	t.Run("Warn", func(t *testing.T) {
		Warn("test message")
		Warnf("formatted %s", "message")
	})

	t.Run("Error", func(t *testing.T) {
		Error("test message")
		Errorf("formatted %s", "message")
	})

	t.Run("Dim", func(t *testing.T) {
		Dim("test message")
		Dimf("formatted %s", "message")
	})

	t.Run("EmptyMessages", func(t *testing.T) {
		Info("")
		Success("")
		Warn("")
		Error("")
		Dim("")
	})

	t.Run("UnicodeMessages", func(t *testing.T) {
		Info("Hello 世界 🌍")
		Success("Completed ✨")
	})
}
