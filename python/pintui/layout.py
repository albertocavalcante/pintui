"""
Layout and structural functions for terminal output.

Provides functions for organizing terminal output with headers,
sections, key-value pairs, steps, dividers, and indentation.
"""

from colorama import init

# Initialize colorama for cross-platform support
init()


def header(title: str) -> None:
    """Print a section header with underline.

    Args:
        title: The header title.

    Example:
        >>> header("Configuration")

        Configuration
        ─────────────
    """
    # Calculate width accounting for unicode characters
    width = len(title)
    # Use box drawing character for underline
    underline = "─" * (width + 8)
    print(f"\n{title}")
    print(underline)


def section(title: str) -> None:
    """Print a subsection title.

    Args:
        title: The section title.

    Example:
        >>> section("Server Settings")

        Server Settings
    """
    print(f"\n{title}")


def kv(key: str, value: str) -> None:
    """Print a key-value pair.

    Args:
        key: The key/label.
        value: The value to display.

    Example:
        >>> kv("Environment", "production")
          Environment: production
    """
    print(f"  {key}: {value}")


def kvf(key: str, fmt: str, *args: object) -> None:
    """Print a key-value pair with formatted value.

    Args:
        key: The key/label.
        fmt: Format string for the value.
        *args: Format arguments.

    Example:
        >>> kvf("Files", "{} processed", 42)
          Files: 42 processed
    """
    kv(key, fmt.format(*args))


def step(current: int, total: int, msg: str) -> None:
    """Print a step indicator.

    Args:
        current: Current step number.
        total: Total number of steps.
        msg: Step description.

    Example:
        >>> step(1, 5, "Initializing")
        [1/5] Initializing
    """
    print(f"[{current}/{total}] {msg}")


def stepf(current: int, total: int, fmt: str, *args: object) -> None:
    """Print a formatted step indicator.

    Args:
        current: Current step number.
        total: Total number of steps.
        fmt: Format string for the message.
        *args: Format arguments.
    """
    step(current, total, fmt.format(*args))


def blank() -> None:
    """Print a blank line for spacing.

    Example:
        >>> blank()

    """
    print()


def divider(width: int = 40) -> None:
    """Print a horizontal divider line.

    Args:
        width: Width of the divider in characters. Defaults to 40.

    Example:
        >>> divider(20)
        ────────────────────
    """
    if width <= 0:
        return
    print("─" * width)


def indent(level: int, msg: str) -> None:
    """Print an indented message.

    Args:
        level: Number of indentation levels (2 spaces each).
        msg: The message to display.

    Example:
        >>> indent(2, "Nested item")
            Nested item
    """
    spaces = "  " * level
    print(f"{spaces}{msg}")


def indentf(level: int, fmt: str, *args: object) -> None:
    """Print a formatted indented message.

    Args:
        level: Number of indentation levels.
        fmt: Format string for the message.
        *args: Format arguments.
    """
    indent(level, fmt.format(*args))
