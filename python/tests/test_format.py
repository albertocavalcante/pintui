"""Tests for pintui.format module."""

from datetime import timedelta

import pytest

from pintui.format import human_duration, human_size, parse_size, pluralize, truncate_path


class TestHumanSize:
    """Tests for human_size function."""

    def test_bytes(self) -> None:
        assert human_size(0) == "0 B"
        assert human_size(100) == "100 B"
        assert human_size(1023) == "1023 B"

    def test_kilobytes(self) -> None:
        assert human_size(1024) == "1.0 KB"
        assert human_size(1536) == "1.5 KB"

    def test_megabytes(self) -> None:
        assert human_size(1024 * 1024) == "1.0 MB"
        assert human_size(1024 * 1024 * 100) == "100.0 MB"

    def test_gigabytes(self) -> None:
        assert human_size(1024 * 1024 * 1024) == "1.0 GB"

    def test_terabytes(self) -> None:
        assert human_size(1024 * 1024 * 1024 * 1024) == "1.00 TB"


class TestParseSize:
    """Tests for parse_size function."""

    def test_bytes(self) -> None:
        assert parse_size("100") == 100
        assert parse_size("100B") == 100
        assert parse_size("100b") == 100

    def test_kilobytes(self) -> None:
        assert parse_size("1KB") == 1024
        assert parse_size("1kb") == 1024

    def test_megabytes(self) -> None:
        assert parse_size("1MB") == 1024 * 1024
        assert parse_size("100MB") == 100 * 1024 * 1024

    def test_gigabytes(self) -> None:
        assert parse_size("1GB") == 1024 * 1024 * 1024

    def test_terabytes(self) -> None:
        assert parse_size("1TB") == 1024 * 1024 * 1024 * 1024

    def test_whitespace(self) -> None:
        assert parse_size("  100MB  ") == 100 * 1024 * 1024

    def test_invalid_empty(self) -> None:
        with pytest.raises(ValueError):
            parse_size("")

    def test_invalid_no_number(self) -> None:
        with pytest.raises(ValueError):
            parse_size("MB")

    def test_invalid_format(self) -> None:
        with pytest.raises(ValueError):
            parse_size("abc")


class TestTruncatePath:
    """Tests for truncate_path function."""

    def test_short_path(self) -> None:
        assert truncate_path("short.txt", 20) == "short.txt"

    def test_exact_length(self) -> None:
        assert truncate_path("exact", 5) == "exact"

    def test_truncate_long_path(self) -> None:
        result = truncate_path("/very/long/path/to/file.txt", 15)
        assert result.startswith("...")
        assert len(result) <= 15

    def test_very_short_max(self) -> None:
        assert truncate_path("test", 3) == "..."
        assert truncate_path("test", 2) == "..."

    def test_empty_path(self) -> None:
        assert truncate_path("", 10) == ""


class TestPluralize:
    """Tests for pluralize function."""

    def test_zero(self) -> None:
        assert pluralize(0, "file", "files") == "0 files"

    def test_one(self) -> None:
        assert pluralize(1, "file", "files") == "1 file"

    def test_multiple(self) -> None:
        assert pluralize(2, "file", "files") == "2 files"
        assert pluralize(100, "item", "items") == "100 items"


class TestHumanDuration:
    """Tests for human_duration function."""

    def test_milliseconds(self) -> None:
        assert human_duration(0) == "0ms"
        assert human_duration(0.5) == "500ms"
        assert human_duration(0.999) == "999ms"

    def test_seconds(self) -> None:
        assert human_duration(1) == "1.0s"
        assert human_duration(5) == "5.0s"
        assert human_duration(5.5) == "5.5s"

    def test_minutes(self) -> None:
        assert human_duration(60) == "1m 0s"
        assert human_duration(90) == "1m 30s"
        assert human_duration(3599) == "59m 59s"

    def test_hours(self) -> None:
        assert human_duration(3600) == "1h 0m"
        assert human_duration(3661) == "1h 1m"
        assert human_duration(7200) == "2h 0m"

    def test_timedelta(self) -> None:
        assert human_duration(timedelta(seconds=90)) == "1m 30s"
        assert human_duration(timedelta(hours=1, minutes=30)) == "1h 30m"
