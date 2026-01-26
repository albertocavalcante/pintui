"""
pintui - A cross-language design system for terminal UIs.

This package provides consistent terminal UI components across different
programming languages, following the pintui design tokens.

Modules:
    messages: Status messages (info, success, warn, error, dim)
    layout: Structural elements (header, section, kv, step, divider)
    progress: Progress indicators (spinner, bar, stages)
    format: Formatting utilities (human_size, parse_size, human_duration)

Example:
    >>> from pintui import messages, layout, progress
    >>>
    >>> messages.info("Starting deployment...")
    >>>
    >>> with progress.spinner("Connecting") as s:
    ...     # do work
    ...     s.success("Connected")
    >>>
    >>> layout.header("Configuration")
    >>> layout.kv("Environment", "production")
"""

from pintui import format, layout, messages, progress

__version__ = "0.1.0"
__all__ = ["messages", "layout", "progress", "format"]
