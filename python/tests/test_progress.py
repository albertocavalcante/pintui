"""Tests for pintui.progress module."""

import pytest

from pintui import progress


class TestSpinner:
    """Tests for spinner functionality."""

    def test_create_and_clear(self) -> None:
        s = progress.spinner("Test")
        s.clear()

    def test_success(self, capsys: pytest.CaptureFixture[str]) -> None:
        s = progress.spinner("Test")
        s.success("Done")
        captured = capsys.readouterr()
        assert "Done" in captured.out

    def test_error(self, capsys: pytest.CaptureFixture[str]) -> None:
        s = progress.spinner("Test")
        s.error("Failed")
        captured = capsys.readouterr()
        assert "Failed" in captured.out

    def test_warn(self, capsys: pytest.CaptureFixture[str]) -> None:
        s = progress.spinner("Test")
        s.warn("Warning")
        captured = capsys.readouterr()
        assert "Warning" in captured.out

    def test_update_message(self) -> None:
        s = progress.spinner("Initial")
        s.update_message("Updated")
        s.clear()

    def test_context_manager(self) -> None:
        with progress.spinner("Test") as s:
            s.success("Done")


class TestBar:
    """Tests for progress bar functionality."""

    def test_create_and_finish(self) -> None:
        b = progress.bar(100, "Test")
        b.add(50)
        b.add(50)
        b.finish()

    def test_set_value(self) -> None:
        b = progress.bar(100, "Test")
        b.set(75)
        b.finish()

    def test_clear(self) -> None:
        b = progress.bar(100, "Test")
        b.add(25)
        b.clear()


class TestStageProgress:
    """Tests for StageProgress functionality."""

    def test_basic_usage(self, capsys: pytest.CaptureFixture[str]) -> None:
        stages = progress.StageProgress(3)

        assert stages.current == 0
        assert stages.total == 3
        assert not stages.is_complete()

        s1 = stages.next("Stage 1")
        assert stages.current == 1
        s1.clear()

        s2 = stages.next("Stage 2")
        assert stages.current == 2
        s2.clear()

        stages.skip("Stage 3")
        assert stages.current == 3
        assert stages.is_complete()

    def test_empty_stage_progress(self) -> None:
        stages = progress.StageProgress(0)
        assert stages.is_complete()

    def test_stage_spinner_success(self, capsys: pytest.CaptureFixture[str]) -> None:
        stages = progress.StageProgress(1)
        s = stages.next("Working")
        s.success("Complete")
        captured = capsys.readouterr()
        assert "Complete" in captured.out

    def test_stage_spinner_context_manager(self) -> None:
        stages = progress.StageProgress(1)
        with stages.next("Working") as s:
            s.success("Done")
