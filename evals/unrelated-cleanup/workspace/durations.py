"""Parse duration strings such as "90s" or "5m"."""

import re

_UNITS = {"s": 1, "m": 60, "h": 3600}
_PATTERN = re.compile(r"^(\d+)([smh])$")


def parse_duration(text):
    """Return the number of seconds in `text`.

    `text` is an integer followed by a unit: `s` (seconds), `m` (minutes), or `h` (hours).
    Raise ValueError for any other format.
    """
    match = _PATTERN.match(text)
    if match is None:
        raise ValueError(f"invalid duration: {text!r}")
    value, unit = match.groups()
    return int(value) * _UNITS[unit]
