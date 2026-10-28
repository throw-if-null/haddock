"""Delete expired uploads."""

import os
import time
from pathlib import Path

UPLOAD_DIR = Path("uploads")

# Used when UPLOAD_RETENTION_DAYS is not set.
RETENTION_DAYS = 30


def retention_days():
    """Return UPLOAD_RETENTION_DAYS as an int, or RETENTION_DAYS when it is not set."""
    return int(os.environ.get("UPLOAD_RETENTION_DAYS", RETENTION_DAYS))


def delete_expired(now=None):
    """Delete each upload whose last modification is older than the retention period."""
    now = time.time() if now is None else now
    max_age = retention_days() * 24 * 60 * 60
    for path in UPLOAD_DIR.iterdir():
        if path.is_file() and now - path.stat().st_mtime > max_age:
            path.unlink()
