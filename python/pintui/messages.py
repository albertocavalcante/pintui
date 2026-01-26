"""
Status message functions for terminal output.

Provides colored, icon-prefixed messages for different status types:
info (blue), success (green), warn (yellow), error (red), and dim (gray).
"""

from colorama import Fore, Style, init

# Initialize colorama for cross-platform support
init()

# Icons matching the design tokens
_INFO_ICON = f"{Fore.BLUE}ℹ{Style.RESET_ALL}"
_SUCCESS_ICON = f"{Fore.GREEN}✓{Style.RESET_ALL}"
_WARN_ICON = f"{Fore.YELLOW}⚠{Style.RESET_ALL}"
_ERROR_ICON = f"{Fore.RED}✗{Style.RESET_ALL}"


def info(msg: str) -> None:
    """Print an info message with blue icon.

    Args:
        msg: The message to display.

    Example:
        >>> info("Starting process...")
        ℹ Starting process...
    """
    print(f"{_INFO_ICON} {msg}")


def infof(fmt: str, *args: object) -> None:
    """Print a formatted info message with blue icon.

    Args:
        fmt: Format string.
        *args: Format arguments.

    Example:
        >>> infof("Processing {} files", 10)
        ℹ Processing 10 files
    """
    info(fmt.format(*args))


def success(msg: str) -> None:
    """Print a success message with green checkmark.

    Args:
        msg: The message to display.

    Example:
        >>> success("Operation completed")
        ✓ Operation completed
    """
    print(f"{_SUCCESS_ICON} {msg}")


def successf(fmt: str, *args: object) -> None:
    """Print a formatted success message with green checkmark.

    Args:
        fmt: Format string.
        *args: Format arguments.
    """
    success(fmt.format(*args))


def warn(msg: str) -> None:
    """Print a warning message with yellow icon.

    Args:
        msg: The message to display.

    Example:
        >>> warn("Rate limit approaching")
        ⚠ Rate limit approaching
    """
    print(f"{_WARN_ICON} {msg}")


def warnf(fmt: str, *args: object) -> None:
    """Print a formatted warning message with yellow icon.

    Args:
        fmt: Format string.
        *args: Format arguments.
    """
    warn(fmt.format(*args))


def error(msg: str) -> None:
    """Print an error message with red X icon.

    Args:
        msg: The message to display.

    Example:
        >>> error("Connection failed")
        ✗ Connection failed
    """
    print(f"{_ERROR_ICON} {msg}")


def errorf(fmt: str, *args: object) -> None:
    """Print a formatted error message with red X icon.

    Args:
        fmt: Format string.
        *args: Format arguments.
    """
    error(fmt.format(*args))


def dim(msg: str) -> None:
    """Print a dimmed message for secondary information.

    Args:
        msg: The message to display.

    Example:
        >>> dim("Debug: internal state updated")
          Debug: internal state updated
    """
    print(f"{Style.DIM}  {msg}{Style.RESET_ALL}")


def dimf(fmt: str, *args: object) -> None:
    """Print a formatted dimmed message.

    Args:
        fmt: Format string.
        *args: Format arguments.
    """
    dim(fmt.format(*args))
