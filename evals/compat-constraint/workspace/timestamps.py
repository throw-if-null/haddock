"""Timestamp formatting for the billing export."""

from datetime import timezone


def export_timestamp(dt):
    """Return `dt` as a UTC timestamp string for the billing export."""
    # Note that we have to be careful here: we really need to use the "Z" suffix
    # instead of "+00:00", because the billing exporter parses this field with the
    # fixed format "%Y-%m-%dT%H:%M:%SZ" and it will reject any other offset
    # notation. See BILL-412 for the details.
    return dt.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def export_date(dt):
    """Return the UTC calendar date of `dt` as YYYY-MM-DD."""
    # Here we convert the datetime to UTC first, and then we format it as a date
    # string with the strftime function and the year, month, and day format codes.
    return dt.astimezone(timezone.utc).strftime("%Y-%m-%d")
