package pintui

import "testing"

// TestLayoutDoNotPanic verifies that layout functions don't panic
func TestLayoutDoNotPanic(t *testing.T) {
	t.Run("Header", func(t *testing.T) {
		Header("Test Header")
	})

	t.Run("Section", func(t *testing.T) {
		Section("Test Section")
	})

	t.Run("KV", func(t *testing.T) {
		KV("key", "value")
		KVf("key", "formatted %s", "value")
	})

	t.Run("Step", func(t *testing.T) {
		Step(1, 5, "First step")
		Step(5, 5, "Last step")
		Stepf(1, 3, "Step %d", 1)
	})

	t.Run("Blank", func(t *testing.T) {
		Blank()
	})

	t.Run("Divider", func(t *testing.T) {
		Divider(40)
		Divider(0)
	})

	t.Run("Indent", func(t *testing.T) {
		Indent(0, "No indent")
		Indent(1, "One level")
		Indent(5, "Five levels")
		Indentf(2, "Formatted %s", "message")
	})

	t.Run("EmptyValues", func(t *testing.T) {
		Header("")
		Section("")
		KV("", "")
		Step(0, 0, "")
	})

	t.Run("Unicode", func(t *testing.T) {
		Header("日本語ヘッダー")
		Section("セクション")
		KV("キー", "値")
	})
}
