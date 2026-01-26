"""Tests for pintui.layout module."""

import pytest

from pintui import layout


class TestLayoutDoNotPanic:
    """Verify that layout functions don't raise exceptions."""

    def test_header(self, capsys: pytest.CaptureFixture[str]) -> None:
        layout.header("Test Header")
        captured = capsys.readouterr()
        assert "Test Header" in captured.out
        assert "─" in captured.out  # Underline

    def test_section(self, capsys: pytest.CaptureFixture[str]) -> None:
        layout.section("Test Section")
        captured = capsys.readouterr()
        assert "Test Section" in captured.out

    def test_kv(self, capsys: pytest.CaptureFixture[str]) -> None:
        layout.kv("key", "value")
        layout.kvf("key", "formatted {}", "value")
        captured = capsys.readouterr()
        assert "key: value" in captured.out
        assert "key: formatted value" in captured.out

    def test_step(self, capsys: pytest.CaptureFixture[str]) -> None:
        layout.step(1, 5, "First step")
        layout.step(5, 5, "Last step")
        layout.stepf(1, 3, "Step {}", 1)
        captured = capsys.readouterr()
        assert "[1/5] First step" in captured.out
        assert "[5/5] Last step" in captured.out
        assert "[1/3] Step 1" in captured.out

    def test_blank(self, capsys: pytest.CaptureFixture[str]) -> None:
        layout.blank()
        captured = capsys.readouterr()
        assert captured.out == "\n"

    def test_divider(self, capsys: pytest.CaptureFixture[str]) -> None:
        layout.divider(40)
        captured = capsys.readouterr()
        assert "─" * 40 in captured.out

    def test_divider_zero(self, capsys: pytest.CaptureFixture[str]) -> None:
        layout.divider(0)
        captured = capsys.readouterr()
        assert captured.out == ""

    def test_indent(self, capsys: pytest.CaptureFixture[str]) -> None:
        layout.indent(0, "No indent")
        layout.indent(1, "One level")
        layout.indent(5, "Five levels")
        layout.indentf(2, "Formatted {}", "message")
        captured = capsys.readouterr()
        assert "No indent" in captured.out
        assert "  One level" in captured.out
        assert "          Five levels" in captured.out
        assert "    Formatted message" in captured.out

    def test_empty_values(self) -> None:
        # Should not raise
        layout.header("")
        layout.section("")
        layout.kv("", "")
        layout.step(0, 0, "")

    def test_unicode(self, capsys: pytest.CaptureFixture[str]) -> None:
        layout.header("日本語ヘッダー")
        layout.section("セクション")
        layout.kv("キー", "値")
        captured = capsys.readouterr()
        assert "日本語ヘッダー" in captured.out
        assert "セクション" in captured.out
        assert "キー: 値" in captured.out
