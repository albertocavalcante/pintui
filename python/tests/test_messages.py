"""Tests for pintui.messages module."""

import pytest

from pintui import messages


class TestMessagesDoNotPanic:
    """Verify that message functions don't raise exceptions."""

    def test_info(self, capsys: pytest.CaptureFixture[str]) -> None:
        messages.info("test message")
        messages.infof("formatted {}", "message")
        captured = capsys.readouterr()
        assert "test message" in captured.out
        assert "formatted message" in captured.out

    def test_success(self, capsys: pytest.CaptureFixture[str]) -> None:
        messages.success("test message")
        messages.successf("formatted {}", "message")
        captured = capsys.readouterr()
        assert "test message" in captured.out
        assert "formatted message" in captured.out

    def test_warn(self, capsys: pytest.CaptureFixture[str]) -> None:
        messages.warn("test message")
        messages.warnf("formatted {}", "message")
        captured = capsys.readouterr()
        assert "test message" in captured.out
        assert "formatted message" in captured.out

    def test_error(self, capsys: pytest.CaptureFixture[str]) -> None:
        messages.error("test message")
        messages.errorf("formatted {}", "message")
        captured = capsys.readouterr()
        assert "test message" in captured.out
        assert "formatted message" in captured.out

    def test_dim(self, capsys: pytest.CaptureFixture[str]) -> None:
        messages.dim("test message")
        messages.dimf("formatted {}", "message")
        captured = capsys.readouterr()
        assert "test message" in captured.out
        assert "formatted message" in captured.out

    def test_empty_messages(self) -> None:
        # Should not raise
        messages.info("")
        messages.success("")
        messages.warn("")
        messages.error("")
        messages.dim("")

    def test_unicode_messages(self, capsys: pytest.CaptureFixture[str]) -> None:
        messages.info("Hello 世界 🌍")
        messages.success("Completed ✨")
        captured = capsys.readouterr()
        assert "Hello 世界 🌍" in captured.out
        assert "Completed ✨" in captured.out
