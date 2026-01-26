"""
Progress indicators for terminal output.

Provides spinner, progress bar, and multi-stage progress tracking
for long-running operations.
"""

from __future__ import annotations

import sys
from typing import TYPE_CHECKING

from colorama import Fore, Style, init
from halo import Halo
from tqdm import tqdm

if TYPE_CHECKING:
    from types import TracebackType

# Initialize colorama
init()

# Icons matching design tokens
_SUCCESS_ICON = f"{Fore.GREEN}✓{Style.RESET_ALL}"
_ERROR_ICON = f"{Fore.RED}✗{Style.RESET_ALL}"
_WARN_ICON = f"{Fore.YELLOW}⚠{Style.RESET_ALL}"
_SKIP_ICON = f"{Style.DIM}○{Style.RESET_ALL}"


class SpinnerHandle:
    """Handle for controlling an active spinner.

    Use the context manager interface or call finish methods directly.

    Example:
        >>> with spinner("Loading") as s:
        ...     # do work
        ...     s.success("Done")

        >>> s = spinner("Loading")
        >>> # do work
        >>> s.success("Done")
    """

    def __init__(self, msg: str) -> None:
        """Create a new spinner.

        Args:
            msg: Initial spinner message.
        """
        self._msg = msg
        self._halo = Halo(
            text=msg,
            spinner="dots",
            color="cyan",
            stream=sys.stdout,
        )
        self._halo.start()

    def __enter__(self) -> SpinnerHandle:
        """Enter context manager."""
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: TracebackType | None,
    ) -> None:
        """Exit context manager, clearing spinner if not already finished."""
        if self._halo.spinner_id is not None:
            self.clear()

    def update_message(self, msg: str) -> None:
        """Update the spinner message.

        Args:
            msg: New message to display.
        """
        self._msg = msg
        self._halo.text = msg

    def success(self, msg: str) -> None:
        """Stop spinner and show success message.

        Args:
            msg: Success message to display.
        """
        self._halo.stop()
        print(f"\r{_SUCCESS_ICON} {msg}")

    def error(self, msg: str) -> None:
        """Stop spinner and show error message.

        Args:
            msg: Error message to display.
        """
        self._halo.stop()
        print(f"\r{_ERROR_ICON} {msg}")

    def warn(self, msg: str) -> None:
        """Stop spinner and show warning message.

        Args:
            msg: Warning message to display.
        """
        self._halo.stop()
        print(f"\r{_WARN_ICON} {msg}")

    def clear(self) -> None:
        """Stop and clear the spinner without a message."""
        self._halo.stop()
        # Clear the line
        print("\r" + " " * (len(self._msg) + 10) + "\r", end="")


def spinner(msg: str) -> SpinnerHandle:
    """Create and start a spinner.

    Args:
        msg: Spinner message.

    Returns:
        SpinnerHandle for controlling the spinner.

    Example:
        >>> with spinner("Connecting") as s:
        ...     # do work
        ...     s.success("Connected")
    """
    return SpinnerHandle(msg)


class BarHandle:
    """Handle for controlling a progress bar.

    Example:
        >>> bar = progress_bar(100, "Downloading")
        >>> for chunk in chunks:
        ...     bar.add(len(chunk))
        >>> bar.finish()
    """

    def __init__(self, total: int, description: str) -> None:
        """Create a new progress bar.

        Args:
            total: Total number of units.
            description: Bar description.
        """
        self._bar = tqdm(
            total=total,
            desc=description,
            bar_format="{desc} {percentage:3.0f}% |{bar}| ({n_fmt}/{total_fmt})",
            ncols=100,
        )

    def add(self, n: int = 1) -> None:
        """Increment the progress bar.

        Args:
            n: Amount to increment by. Defaults to 1.
        """
        self._bar.update(n)

    def set(self, n: int) -> None:
        """Set the progress bar to a specific value.

        Args:
            n: Value to set.
        """
        self._bar.n = n
        self._bar.refresh()

    def finish(self) -> None:
        """Complete and close the progress bar."""
        self._bar.close()

    def clear(self) -> None:
        """Clear the progress bar without completing."""
        self._bar.close()


def bar(total: int, description: str = "") -> BarHandle:
    """Create a progress bar.

    Args:
        total: Total number of units.
        description: Bar description.

    Returns:
        BarHandle for controlling the progress bar.

    Example:
        >>> b = bar(100, "Processing")
        >>> for i in range(100):
        ...     b.add(1)
        >>> b.finish()
    """
    return BarHandle(total, description)


class StageProgress:
    """Track progress through multiple stages.

    Provides a way to show progress through a series of steps,
    with automatic stage numbering.

    Example:
        >>> stages = StageProgress(3)
        >>> with stages.next("Compiling") as s:
        ...     # do work
        ...     s.success("Compiled")
        >>> with stages.next("Testing") as s:
        ...     s.success("All tests pass")
        >>> stages.skip("Deployment")
    """

    def __init__(self, total: int) -> None:
        """Create a new stage progress tracker.

        Args:
            total: Total number of stages.
        """
        self._total = total
        self._current = 0

    @property
    def current(self) -> int:
        """Get the current stage number (0-indexed before any stages run)."""
        return self._current

    @property
    def total(self) -> int:
        """Get the total number of stages."""
        return self._total

    def is_complete(self) -> bool:
        """Check if all stages are complete."""
        return self._current >= self._total

    def next(self, msg: str) -> StageSpinnerHandle:
        """Start the next stage with a spinner.

        Args:
            msg: Stage description.

        Returns:
            StageSpinnerHandle for controlling the stage spinner.
        """
        self._current += 1
        return StageSpinnerHandle(self._current, self._total, msg)

    def skip(self, msg: str) -> None:
        """Skip the next stage.

        Args:
            msg: Stage description.
        """
        self._current += 1
        skipped = f"{Style.DIM}(skipped){Style.RESET_ALL}"
        print(f"  {_SKIP_ICON} [{self._current}/{self._total}] {msg} {skipped}")


class StageSpinnerHandle:
    """Handle for a spinner within a stage progress.

    Similar to SpinnerHandle but includes stage numbering.
    """

    def __init__(self, current: int, total: int, msg: str) -> None:
        """Create a stage spinner.

        Args:
            current: Current stage number.
            total: Total stages.
            msg: Stage message.
        """
        self._current = current
        self._total = total
        self._msg = msg
        self._prefix = f"[{current}/{total}]"
        self._halo = Halo(
            text=f"{self._prefix} {msg}",
            spinner="dots",
            color="cyan",
            stream=sys.stdout,
        )
        self._halo.start()

    def __enter__(self) -> StageSpinnerHandle:
        """Enter context manager."""
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: TracebackType | None,
    ) -> None:
        """Exit context manager."""
        if self._halo.spinner_id is not None:
            self.clear()

    def update_message(self, msg: str) -> None:
        """Update the stage message.

        Args:
            msg: New message.
        """
        self._msg = msg
        self._halo.text = f"{self._prefix} {msg}"

    def success(self, msg: str) -> None:
        """Complete stage with success.

        Args:
            msg: Success message.
        """
        self._halo.stop()
        print(f"\r{_SUCCESS_ICON} {self._prefix} {msg}")

    def error(self, msg: str) -> None:
        """Complete stage with error.

        Args:
            msg: Error message.
        """
        self._halo.stop()
        print(f"\r{_ERROR_ICON} {self._prefix} {msg}")

    def warn(self, msg: str) -> None:
        """Complete stage with warning.

        Args:
            msg: Warning message.
        """
        self._halo.stop()
        print(f"\r{_WARN_ICON} {self._prefix} {msg}")

    def clear(self) -> None:
        """Clear the spinner without a message."""
        self._halo.stop()
        full_msg = f"{self._prefix} {self._msg}"
        print("\r" + " " * (len(full_msg) + 10) + "\r", end="")
