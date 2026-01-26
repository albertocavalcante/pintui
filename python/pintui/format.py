"""
Formatting utilities for terminal output.

Provides human-readable formatting for sizes, durations, counts,
and path truncation.
"""

import re
from datetime import timedelta

# Size constants
KB = 1024
MB = KB * 1024
GB = MB * 1024
TB = GB * 1024


def human_size(bytes_: int) -> str:
    """Format a byte count as a human-readable string.

    Args:
        bytes_: Number of bytes.

    Returns:
        Human-readable size string.

    Example:
        >>> human_size(1024)
        '1.0 KB'
        >>> human_size(1024 * 1024 * 100)
        '100.0 MB'
    """
    if bytes_ >= TB:
        return f"{bytes_ / TB:.2f} TB"
    if bytes_ >= GB:
        return f"{bytes_ / GB:.1f} GB"
    if bytes_ >= MB:
        return f"{bytes_ / MB:.1f} MB"
    if bytes_ >= KB:
        return f"{bytes_ / KB:.1f} KB"
    return f"{bytes_} B"


def parse_size(size_str: str) -> int:
    """Parse a size string into bytes.

    Supports B, KB, MB, GB, TB suffixes (case-insensitive).

    Args:
        size_str: Size string like "100MB" or "1.5GB".

    Returns:
        Size in bytes.

    Raises:
        ValueError: If the string cannot be parsed.

    Example:
        >>> parse_size("100MB")
        104857600
        >>> parse_size("1KB")
        1024
    """
    size_str = size_str.strip().upper()
    if not size_str:
        raise ValueError("Empty size string")

    # Match number and optional unit
    match = re.match(r"^(\d+(?:\.\d+)?)\s*(B|KB|MB|GB|TB)?$", size_str)
    if not match:
        raise ValueError(f"Invalid size format: {size_str}")

    value = float(match.group(1))
    unit = match.group(2) or "B"

    multipliers = {
        "B": 1,
        "KB": KB,
        "MB": MB,
        "GB": GB,
        "TB": TB,
    }

    return int(value * multipliers[unit])


def human_duration(duration: timedelta | float) -> str:
    """Format a duration as a human-readable string.

    Args:
        duration: Duration as timedelta or seconds (float).

    Returns:
        Human-readable duration string.

    Example:
        >>> human_duration(timedelta(seconds=90))
        '1m 30s'
        >>> human_duration(0.5)
        '500ms'
    """
    if isinstance(duration, timedelta):
        total_seconds = duration.total_seconds()
    else:
        total_seconds = duration

    if total_seconds < 0:
        total_seconds = 0

    # Convert to milliseconds for precision
    total_ms = int(total_seconds * 1000)

    if total_ms < 1000:
        return f"{total_ms}ms"

    total_secs = total_seconds
    if total_secs < 60:
        return f"{total_secs:.1f}s"

    minutes = int(total_secs // 60)
    secs = int(total_secs % 60)

    if minutes < 60:
        return f"{minutes}m {secs}s"

    hours = minutes // 60
    mins = minutes % 60
    return f"{hours}h {mins}m"


def pluralize(count: int, singular: str, plural: str) -> str:
    """Return a pluralized string based on count.

    Args:
        count: The count.
        singular: Singular form.
        plural: Plural form.

    Returns:
        String with count and appropriate form.

    Example:
        >>> pluralize(1, "file", "files")
        '1 file'
        >>> pluralize(5, "file", "files")
        '5 files'
    """
    if count == 1:
        return f"{count} {singular}"
    return f"{count} {plural}"


def truncate_path(path: str, max_len: int) -> str:
    """Truncate a path to fit within max_len characters.

    If truncation is needed, replaces the beginning with "...".

    Args:
        path: The path to truncate.
        max_len: Maximum length.

    Returns:
        Truncated path string.

    Example:
        >>> truncate_path("/very/long/path/to/file.txt", 15)
        '.../to/file.txt'
    """
    if not path:
        return ""

    if len(path) <= max_len:
        return path

    if max_len <= 3:
        return "..."

    # Find a good break point (path separator)
    suffix_len = max_len - 3  # Account for "..."
    suffix = path[-suffix_len:]

    # Try to break at a path separator for cleaner output
    sep_idx = suffix.find("/")
    if sep_idx > 0 and sep_idx < len(suffix) - 1:
        suffix = suffix[sep_idx:]

    return f"...{suffix}"
