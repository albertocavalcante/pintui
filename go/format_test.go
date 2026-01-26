package pintui

import (
	"testing"
	"time"
)

func TestHumanSize(t *testing.T) {
	tests := []struct {
		bytes    uint64
		expected string
	}{
		{0, "0 B"},
		{100, "100 B"},
		{1023, "1023 B"},
		{1024, "1.0 KB"},
		{1536, "1.5 KB"},
		{1024 * 1024, "1.0 MB"},
		{1024 * 1024 * 100, "100.0 MB"},
		{1024 * 1024 * 1024, "1.0 GB"},
		{1024 * 1024 * 1024 * 1024, "1.00 TB"},
	}

	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			got := HumanSize(tt.bytes)
			if got != tt.expected {
				t.Errorf("HumanSize(%d) = %q, want %q", tt.bytes, got, tt.expected)
			}
		})
	}
}

func TestParseSize(t *testing.T) {
	tests := []struct {
		input    string
		expected uint64
		wantErr  bool
	}{
		{"100", 100, false},
		{"100B", 100, false},
		{"100b", 100, false},
		{"1KB", 1024, false},
		{"1kb", 1024, false},
		{"1MB", 1024 * 1024, false},
		{"100MB", 100 * 1024 * 1024, false},
		{"1GB", 1024 * 1024 * 1024, false},
		{"1TB", 1024 * 1024 * 1024 * 1024, false},
		{"  100MB  ", 100 * 1024 * 1024, false},
		{"", 0, true},
		{"abc", 0, true},
		{"MB", 0, true},
		{"-100MB", 0, true},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			got, err := ParseSize(tt.input)
			if (err != nil) != tt.wantErr {
				t.Errorf("ParseSize(%q) error = %v, wantErr %v", tt.input, err, tt.wantErr)
				return
			}
			if !tt.wantErr && got != tt.expected {
				t.Errorf("ParseSize(%q) = %d, want %d", tt.input, got, tt.expected)
			}
		})
	}
}

func TestTruncatePath(t *testing.T) {
	tests := []struct {
		path     string
		maxLen   int
		expected string
	}{
		{"short.txt", 20, "short.txt"},
		{"exact", 5, "exact"},
		{"/very/long/path/to/file.txt", 15, ".../to/file.txt"},
		{"test", 3, "..."},
		{"test", 2, "..."},
		{"", 10, ""},
	}

	for _, tt := range tests {
		t.Run(tt.path, func(t *testing.T) {
			got := TruncatePath(tt.path, tt.maxLen)
			if got != tt.expected {
				t.Errorf("TruncatePath(%q, %d) = %q, want %q", tt.path, tt.maxLen, got, tt.expected)
			}
		})
	}
}

func TestPluralize(t *testing.T) {
	tests := []struct {
		count    int
		singular string
		plural   string
		expected string
	}{
		{0, "file", "files", "0 files"},
		{1, "file", "files", "1 file"},
		{2, "file", "files", "2 files"},
		{100, "item", "items", "100 items"},
	}

	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			got := Pluralize(tt.count, tt.singular, tt.plural)
			if got != tt.expected {
				t.Errorf("Pluralize(%d, %q, %q) = %q, want %q",
					tt.count, tt.singular, tt.plural, got, tt.expected)
			}
		})
	}
}

func TestHumanDuration(t *testing.T) {
	tests := []struct {
		duration time.Duration
		expected string
	}{
		{0, "0ms"},
		{500 * time.Millisecond, "500ms"},
		{999 * time.Millisecond, "999ms"},
		{1 * time.Second, "1.0s"},
		{5 * time.Second, "5.0s"},
		{5500 * time.Millisecond, "5.5s"},
		{60 * time.Second, "1m 0s"},
		{90 * time.Second, "1m 30s"},
		{3599 * time.Second, "59m 59s"},
		{3600 * time.Second, "1h 0m"},
		{3661 * time.Second, "1h 1m"},
		{7200 * time.Second, "2h 0m"},
	}

	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			got := HumanDuration(tt.duration)
			if got != tt.expected {
				t.Errorf("HumanDuration(%v) = %q, want %q", tt.duration, got, tt.expected)
			}
		})
	}
}
