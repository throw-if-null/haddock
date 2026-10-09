"""Helpers for ordering and paging account records."""


def ordered(records):
    """Return the records sorted by creation time, oldest first."""
    # list.sort() sorts in place, so copy the input first. Callers pass the
    # list they cache, and an in-place sort would reorder their copy.
    items = list(records)
    items.sort(key=lambda record: record.created_at)
    return items


def page(records, number, size=50):
    """Return page `number` (starting at 1) of `records`, `size` items per page."""
    start = (number - 1) * size
    return records[start:start + size]
