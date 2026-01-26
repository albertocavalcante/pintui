package pintui

import "testing"

func TestSpinner(t *testing.T) {
	t.Run("CreateAndClear", func(t *testing.T) {
		s := Spinner("Test")
		s.Clear()
	})

	t.Run("Success", func(t *testing.T) {
		s := Spinner("Test")
		s.Success("Done")
	})

	t.Run("Error", func(t *testing.T) {
		s := Spinner("Test")
		s.Error("Failed")
	})

	t.Run("Warn", func(t *testing.T) {
		s := Spinner("Test")
		s.Warn("Warning")
	})

	t.Run("UpdateMessage", func(t *testing.T) {
		s := Spinner("Initial")
		s.UpdateMessage("Updated")
		s.Clear()
	})
}

func TestBar(t *testing.T) {
	t.Run("CreateAndFinish", func(t *testing.T) {
		bar := Bar(100, "Test")
		bar.Add(50)
		bar.Add(50)
		bar.Clear()
	})
}

func TestStageProgress(t *testing.T) {
	t.Run("BasicUsage", func(t *testing.T) {
		stages := NewStageProgress(3)

		if stages.Current() != 0 {
			t.Errorf("Current() = %d, want 0", stages.Current())
		}
		if stages.Total() != 3 {
			t.Errorf("Total() = %d, want 3", stages.Total())
		}
		if stages.IsComplete() {
			t.Error("IsComplete() = true, want false")
		}

		s1 := stages.Next("Stage 1")
		if stages.Current() != 1 {
			t.Errorf("Current() = %d, want 1", stages.Current())
		}
		s1.Clear()

		s2 := stages.Next("Stage 2")
		if stages.Current() != 2 {
			t.Errorf("Current() = %d, want 2", stages.Current())
		}
		s2.Clear()

		stages.Skip("Stage 3")
		if stages.Current() != 3 {
			t.Errorf("Current() = %d, want 3", stages.Current())
		}
		if !stages.IsComplete() {
			t.Error("IsComplete() = false, want true")
		}
	})

	t.Run("EmptyStageProgress", func(t *testing.T) {
		stages := NewStageProgress(0)
		if !stages.IsComplete() {
			t.Error("IsComplete() = false, want true for 0 stages")
		}
	})
}
